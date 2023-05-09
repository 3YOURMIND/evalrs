FROM rust:1.69 as builder
WORKDIR app
RUN rustup update
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc as runtime
COPY --from=builder /app/target/release/evalrs /
COPY --from=builder /app/config /config
COPY --from=builder /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/deno_core-0.114.0/*.js /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/deno_core-0.114.0/
ENTRYPOINT ["./evalrs"]
