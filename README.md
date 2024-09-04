rustup target add thumbv7em-none-eabihf
rustup component add rust-analyzer
rustup component add llvm-tools
cargo install cargo-binutils
cargo size -- -Ax
sudo cp 69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload
sudo udevadm trigger
