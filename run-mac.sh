set -e
cd sandbox
cargo build --release
cd ..
rm -f sandbox_py.so
ln -s sandbox/target/release/libsandbox_py.dylib ./sandbox_py.so
python3 generate_paths.py