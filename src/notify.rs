use libnotify::Notification;

use crate::errors::Error;

pub fn low_battery_notification() -> Notification {
    let title = "Low Battery";
    let body = "Battery is currently low.";
    let notification = libnotify::Notification::new(title, Some(body), None);
    notification.set_timeout(0);
    notification
}

pub fn show(notification: &Notification) -> Result<(), Error> {
    notification.show().map_err(|_| Error::NotificationError)?;
    Ok(())
}

pub fn init(app_name: &str) {
    libnotify::init(app_name).unwrap();
}
