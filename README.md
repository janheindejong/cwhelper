# Crossword Helper 

See justfile for commands. 

## Requirements 

For basic development: 

* [Rust & Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) 
* [Just](https://github.com/casey/just)

For cross-compilation for Raspberry Pi: 

```sh
sudo apt install gcc-aarch64-linux-gnu # Install cross-compilation linker
rustup target add aarch64-unknown-linux-gnu # Register as target for the rust compiler 
```
