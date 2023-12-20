# Battery Monitor Daemon

Simple script to alert when battery is low.

Requires libnotify and upower.

## Install

```
cargo install --path . --root /path/to/install/dir
systemctl enable --user ./batteryd.service
```
