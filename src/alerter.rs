use futures::stream::StreamExt;
use futures_time::time::Duration;
use upower_dbus::{DeviceProxy, UPowerProxy};

use crate::notify;

pub async fn init() -> zbus::Result<UPowerProxy<'static>> {
    let connection = zbus::Connection::system().await?;
    UPowerProxy::new(&connection).await
}

enum BatteryState {
    Charging,
    NotCharging,
}

async fn sleep(duration: Duration) {
    futures_time::task::sleep(duration).await;
}

pub struct Config {
    pub alert_threshold: f64,
    pub normal_sleep: Duration,
    pub long_sleep: Duration,
}

pub struct BatteryAlerter {
    upower: UPowerProxy<'static>,
    device: DeviceProxy<'static>,
    config: Config,
}

impl BatteryAlerter {
    pub async fn new(upower: UPowerProxy<'static>, config: Config) -> Self {
        let device = upower.get_display_device().await.unwrap();
        BatteryAlerter {
            upower,
            device,
            config,
        }
    }

    pub async fn start(&self) {
        self.wait_for_state(BatteryState::NotCharging).await;
        loop {
            self.check_battery().await;
        }
    }

    async fn wait_for_state(&self, state: BatteryState) {
        let mut stream = self.upower.receive_on_battery_changed().await;

        let waiting_for_battery = matches!(state, BatteryState::NotCharging);
        let reached_state = |on_battery| on_battery == waiting_for_battery;

        while let Some(event) = stream.next().await {
            if let Ok(charging) = event.get().await {
                if reached_state(charging) {
                    break;
                }
            }
        }
    }

    async fn check_battery(&self) {
        match self.device.percentage().await {
            Ok(percentage) if percentage < self.config.alert_threshold => {
                let notification = notify::notify(percentage).unwrap();
                self.wait_for_state(BatteryState::Charging).await;
                let _ = notification.close();
                self.wait_for_state(BatteryState::NotCharging).await;
            }
            Ok(_) => sleep(self.config.normal_sleep).await,
            Err(err) => {
                println!("Error getting battery status: {err:?}");
                sleep(self.config.long_sleep).await;
            }
        };
    }
}
