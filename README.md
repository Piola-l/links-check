# Links-check

A simple rust program that allows you view links responce status. Returns result as a notification

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
