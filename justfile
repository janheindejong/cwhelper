fmt: 
    cargo fmt

lint: 
    cargo fmt --check

check: lint
    cargo test --lib

build-cli: check 
    cargo build --release --bin econogram-helper

try-cli: check
    cargo run --bin econogram-helper t*st

try-web: check
    cargo watch -c -w src -w static -x 'run --bin web'
