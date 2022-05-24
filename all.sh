echo "Checking formatting..."
cargo fmt --all --check &&
echo "Checking clippy warnings..."
cargo clippy -- -D warnings &&
echo "Running tests..."
cargo test
