set -e
cd sandbox
cargo build --release
cd ..
python3 app.py