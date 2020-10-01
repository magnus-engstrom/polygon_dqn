set -e
cd sandbox
cargo build --release
cd ..
rm -f sandbox.so
ln -s sandbox/target/release/libsandbox.dylib ./sandbox.so
python3 app.py