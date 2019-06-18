cargo build --release
cargo run --release -- --host &
sleep 5
cargo run --release -- --join 127.0.0.1:62304 &
