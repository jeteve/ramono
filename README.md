# About Ramono

```ramono``` is a small program that greedily consumes resources (RAM, Open Files..) in a controlled way.

It is useful to validate that your infrastructure can cope with problematic
containers that would escape their resource  allocations.

For now ramono is capable of consuming:

- RAM
- File handles

Other types of resources will be implemented in future versions, with a focus on the resources
that can be controlled with ```ulimit```.

# Usage

```sh
$ docker run jeteve/ramono

Usage: ramono [OPTIONS]

Options:
  -m, --mem-increment <MEMORY_INCREMENT>
          [default: 1000]
  -l, --mem-limit <MEMORY_LIMIT>
          [default: 1000000000]
      --file-increment <FILE_INCREMENT>
          [default: 0]
      --file-limit <FILE_LIMIT>
          [default: 1000000000]
  -v, --verbose-level <VERBOSE_LEVEL>
          [default: info] [possible values: trace, debug, info, warn, error]
  -h, --help
          Print help
  -V, --version
          Print version
```

# Installing

For now, this is only available as a docker image:

https://hub.docker.com/r/jeteve/ramono


# Development

## Notes on compiling

Compiling for alpine and other MUSL based distribs (see https://musl.libc.org/)

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

## Releasing

For a minor version bump:
```sh
 cargo install cargo-release
 cargo release -v --no-publish minor
```
