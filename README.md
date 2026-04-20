# Crossword Helper 

A little tool to help you when you get stuck with a crossword puzzle. 

## Running 

The logic can be accessed in two ways: 

* *cli*: cargo run --bin cwhelper -- "t*st"
* *web*: cargo run --bin web

For more commands, see the (justfile)[./justfile].

## Requirements 

For basic development: 

* [Rust & Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) 
* [Just](https://github.com/casey/just)

For cross-compilation for Raspberry Pi: 

```sh
sudo apt install gcc-aarch64-linux-gnu # Install cross-compilation linker
rustup target add aarch64-unknown-linux-gnu # Register as target for the rust compiler 
```
