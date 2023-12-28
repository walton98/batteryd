# Battery Monitor Daemon

Simple script to alert when battery is low.

Requires libnotify and upower.

[![Battery Monitor CI](https://github.com/walton98/batteryd/actions/workflows/ci.yml/badge.svg)](https://github.com/walton98/batteryd/actions/workflows/ci.yml)

## Install

```
cargo install --path . --root /path/to/install/dir
systemctl enable --user ./batteryd.service
```
