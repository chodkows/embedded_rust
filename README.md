# Config environment to support embedded rust
```
rustup target add thumbv7em-none-eabihf
rustup component add rust-analyzer
rustup component add llvm-tools
cargo install cargo-binutils
cargo size -- -Ax
```


# Update linux to support flashing firmware
https://probe.rs/docs/getting-started/probe-setup/#linux%3A-udev-rules
```
sudo cp 69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload
sudo udevadm trigger
```
