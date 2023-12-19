use futures::future::select;
use futures::future::Either;

use futures::stream::StreamExt;
use futures_time::time::Duration;
use upower_dbus::{BatteryState, DeviceProxy, UPowerProxy};
use zbus::PropertyStream;

use crate::notify;

pub async fn init() -> zbus::Result<UPowerProxy<'static>> {
    let connection = zbus::Connection::system().await?;
    UPowerProxy::new(&connection).await
}

pub struct Config {
    pub alert_threshold: f64,
    pub normal_sleep: Duration,
    pub long_sleep: Duration,
}

pub struct BatteryAlerter {
    device: DeviceProxy<'static>,
    config: Config,
}

impl BatteryAlerter {
    pub fn new(device: DeviceProxy<'static>, config: Config) -> Self {
        BatteryAlerter { device, config }
    }

    pub async fn start(&self) {
        loop {
            self.wait_for_discharging().await;
            let percent_fut = Box::pin(self.wait_for_percentage());
            let charge_fut = Box::pin(self.wait_for_charging());
            match select(percent_fut, charge_fut).await {
                Either::Left((_, charge_fut)) => {
                    let notification = notify::low_battery_notification();
                    notify::show(&notification).unwrap();
                    charge_fut.await;
                    let _ = notification.close();
                }
                Either::Right((_, _)) => (),
            };
        }
    }

    async fn wait_for_discharging(&self) {
        let predicate = |state| matches!(state, BatteryState::Discharging);
        self.wait_for_battery_state(predicate).await;
    }

    async fn wait_for_charging(&self) {
        let predicate =
            |state| matches!(state, BatteryState::PendingCharge | BatteryState::Charging);
        self.wait_for_battery_state(predicate).await;
    }

    async fn wait_for_battery_state<F>(&self, predicate: F)
    where
        F: Fn(BatteryState) -> bool,
    {
        let state = self.device.state().await;
        let stream = self.device.receive_state_changed().await;
        wait_for(state, stream, predicate).await;
    }

    async fn wait_for_percentage(&self) {
        let state = self.device.percentage().await;
        let stream = self.device.receive_percentage_changed().await;
        wait_for(state, stream, |percentage| {
            percentage < self.config.alert_threshold
        })
        .await;
    }
}

async fn wait_for<T, F>(current: zbus::Result<T>, mut stream: PropertyStream<'_, T>, predicate: F)
where
    T: Unpin + TryFrom<zvariant::OwnedValue>,
    T::Error: Into<zbus::Error>,
    F: Fn(T) -> bool,
{
    let completed = |value| match value {
        Ok(value) => predicate(value),
        Err(_) => false,
    };
    if completed(current) {
        return;
    }
    while let Some(event) = stream.next().await {
        if completed(event.get().await) {
            return;
        }
    }
}
