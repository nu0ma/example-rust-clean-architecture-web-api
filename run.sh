cargo fmt
cargo test
cargo clippy --fix

DB_HOST=localhost \
DB_PORT=5432 \
DB_USER=numa \
DB_PASSWORD=password \
DB_NAME=example \
RUST_LOG=DEBUG cargo r 