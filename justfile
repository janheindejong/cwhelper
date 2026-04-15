build: 
    cargo build --release

try: build
    ./target/release/econogram-helper t*st --lexicon wordlist.txt