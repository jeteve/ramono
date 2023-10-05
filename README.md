# Ramono

```ramono``` is a small program that consumes resources

# Run it

This is meant to be made available as a docker image.

# Notes

Compiling for alpine and other MUSL based distribs (see https://musl.libc.org/)

```
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

```
