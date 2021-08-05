#sudo systemctl start docker.service
#cargo install cross
mkdir -p bin
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
cross build --release --target aarch64-linux-android
cross build --release --target x86_64-pc-windows-gnu

cp target/x86_64-unknown-linux-gnu/release/rrtop bin/rrtop-x86_64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/rrtop bin/rrtop-aarch64-unknown-linux-gnu
cp target/aarch64-linux-android/release/rrtop bin/rrtop-aarch64-linux-android
cp target/x86_64-pc-windows-gnu/release/rrtop.exe bin/rrtop-x86_64-pc-windows-gnu.exe
