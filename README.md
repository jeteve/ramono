# Ramono

```ramono``` is a small program that consumes resources

# Run it

This is available as a docker image:

https://hub.docker.com/r/jeteve/ramono


# Development

# Notes on compiling

Compiling for alpine and other MUSL based distribs (see https://musl.libc.org/)

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```
