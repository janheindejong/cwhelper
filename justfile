RASPBERRYPI_HOST := "192.168.2.21"
WINDOWS_PATH := "/mnt/c/Users/JanHeindeJong/.local/bin"

fmt: 
    cargo fmt

lint: 
    cargo fmt --check

check: lint
    cargo test

run word="t*st": 
    cargo run --bin cwhelper {{word}}

run-web: 
    cargo watch -c -w src -w static -x 'run --bin web'

build-cli target="": check 
    cargo build --release --bin cwhelper {{ if target != "" { "--target " + target } else { "" } }}

build-web target="": check
    cargo build --release --bin web {{ if target != "" { "--target " + target } else { "" } }}

deploy-to-pi: (build-web "aarch64-unknown-linux-gnu")
    ssh janhein@{{RASPBERRYPI_HOST}} 'sudo systemctl stop cwhelper'
    scp target/aarch64-unknown-linux-gnu/release/web janhein@{{RASPBERRYPI_HOST}}:~/cwhelper/web >/dev/null
    ssh janhein@{{RASPBERRYPI_HOST}} 'sudo systemctl start cwhelper'

deploy-to-windows: (build-cli "x86_64-pc-windows-gnu") (build-web "x86_64-pc-windows-gnu")
    cp target/x86_64-pc-windows-gnu/release/web.exe {{WINDOWS_PATH}}/cwhelper-web.exe
    cp target/x86_64-pc-windows-gnu/release/cwhelper.exe {{WINDOWS_PATH}}/cwhelper.exe
