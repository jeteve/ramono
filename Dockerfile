FROM rust as builder

WORKDIR /work/
COPY . .
# Compile everything statically
RUN rustup target add x86_64-unknown-linux-musl
RUN ulimit -n 10000 && cargo test
RUN cargo build --release --target x86_64-unknown-linux-musl 

RUN wget https://github.com/upx/upx/releases/download/v4.2.4/upx-4.2.4-amd64_linux.tar.xz && \
    tar -xvf upx-4.2.4-amd64_linux.tar.xz && \
    upx-4.2.4-amd64_linux/upx -9 target/x86_64-unknown-linux-musl/release/ramono

FROM scratch
COPY --from=builder /work/target/x86_64-unknown-linux-musl/release/ramono /ramono
ENTRYPOINT ["/ramono"]
CMD ["--help"]
