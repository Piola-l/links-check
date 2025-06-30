# Links-check

A simple rust utility that allows you view links responce status. Returns result as a notification

## Run
``` sh
cargo run -- https://example.com https://httpbin.org/status/404
```

## Build
``` sh
cargo build --release
```

## Install

### System-wide:
``` sh
sudo cp target/release/links-check /usr/local/bin/
```
### User-only:
``` sh
cp target/release/links-check ~/.local/bin
```

## Systemd service

Create service
``` sh
mkdir -p ~/.config/systemd/user
nano ~/.config/systemd/user/links_check.service
```

Paste this:
``` txt
[Unit]
Description=Links responce check app

[Service]
ExecStart=/bin/bash -c "cat ~/links_check | links-check"
Restart=on-failure

[Install]
WantedBy=default.target
```
This puts all links from the ~/links_check file to the program.

Enable service
``` sh
systemctl --user daemon-reexec
systemctl --user daemon-reload
systemctl --user enable links_check.service
systemctl --user start links_check.service
```


