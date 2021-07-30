RUSTFLAGS="-C target-cpu=native" cargo build --release
sudo cp target/release/rrtop /bin/rrtop