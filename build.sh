cargo build --release
tar -czvf release/macos.zip ./README.md -C target/release/ $(basename $(pwd))

cargo build --release --target x86_64-pc-windows-gnu
tar -czvf release/windows.zip ./README.md -C target/x86_64-pc-windows-gnu/release/ $(basename $(pwd)).exe
