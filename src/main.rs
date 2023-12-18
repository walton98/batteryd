mod alerter;
mod errors;
mod notify;

use futures_time::time::Duration;

use crate::alerter::{BatteryAlerter, Config};

async fn run() {
    notify::init("battery_alerter");
    let upower = alerter::init().await.unwrap();

    let config = Config {
        alert_threshold: 10.into(),
        normal_sleep: Duration::from_secs(1),
        long_sleep: Duration::from_secs(10),
    };

    BatteryAlerter::new(upower, config).await.start().await;
}

fn main() {
    futures::executor::block_on(async move {
        run().await;
    });
}
