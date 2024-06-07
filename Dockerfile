FROM rust as builder

WORKDIR /work/
COPY . .
# Compile everything statically
RUN rustup target add x86_64-unknown-linux-musl
RUN ulimit -n 10000 && cargo test
RUN cargo build --release --target x86_64-unknown-linux-musl 

FROM scratch
COPY --from=builder /work/target/x86_64-unknown-linux-musl/release/ramono /ramono
ENTRYPOINT ["/ramono"]
CMD ["--help"]
