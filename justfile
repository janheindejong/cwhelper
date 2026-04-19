fmt: 
    cargo fmt

lint: 
    cargo fmt --check

check: lint
    cargo test --lib

build: check 
    cargo build --release

try: build
    ./target/release/econogram-helper t*st