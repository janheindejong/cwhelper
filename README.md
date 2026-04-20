# Crossword Helper 

A little tool to help you when you get stuck with a crossword puzzle. 

## Running 

The logic can be accessed in two ways: 

* **As a CLI**: `just run "c*fe"`
* **As a web-service**: `just run-web`

For more commands, see the [justfile](./justfile).

## Requirements 

For basic development: 

* [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) 
* [Just](https://github.com/casey/just)

For cross-compilation for Raspberry Pi: 

```sh
sudo apt install gcc-aarch64-linux-gnu # Install cross-compilation linker
rustup target add aarch64-unknown-linux-gnu # Register as target for the rust compiler 
```

## Roadmap 

English lexicon is on the to-do list, then we'll publish on r/crossword to see what's up. 
