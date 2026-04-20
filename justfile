RASPBERRYPI_HOST := "192.168.2.21"

fmt: 
    cargo fmt

lint: 
    cargo fmt --check

check: lint
    cargo test --lib

try-cli: check
    cargo run --bin cwhelper t*st

try-web: check
    cargo watch -c -w src -w static -x 'run --bin web'

build-cli: check 
    cargo build --release --bin cwhelper

build-web: check
    cargo build --release --bin web --target aarch64-unknown-linux-gnu

deploy-web: build-web
    scp target/aarch64-unknown-linux-gnu/release/web janhein@{{RASPBERRYPI_HOST}}:~/cwhelper/web >/dev/null
