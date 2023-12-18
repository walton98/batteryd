use libnotify::Notification;

use crate::errors::Error;

pub fn notify(percentage: f64) -> Result<Notification, Error> {
    let title = "Low Battery";
    let body: &str = &format!("Battery is currently at {percentage}%.");
    let n = libnotify::Notification::new(title, Some(body), None);
    n.show().map_err(|_| Error::NotificationError)?;
    Ok(n)
}

pub fn init(app_name: &str) {
    libnotify::init(app_name).unwrap();
}
