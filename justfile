fmt: 
    cargo fmt

lint: 
    cargo fmt --check

check: lint
    cargo test --lib

build-cli: check 
    cargo build --release --bin cwhelper

try-cli: check
    cargo run --bin cwhelper t*st

try-web: check
    cargo watch -c -w src -w static -x 'run --bin web'
