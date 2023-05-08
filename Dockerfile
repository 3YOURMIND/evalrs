FROM rust:1.69 as planner
WORKDIR app
RUN cargo install cargo-chef --version 0.1.59
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef --version 0.1.59
COPY --from=planner /app/recipe.json recipe.json
RUN rustup update
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN rustup update
RUN cargo build --release

FROM gcr.io/distroless/cc as runtime
COPY --from=builder /app/target/release/evalrs /
COPY --from=builder /app/config /config
COPY --from=builder /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/deno_core-0.114.0/*.js /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/deno_core-0.114.0/
ENTRYPOINT ["./evalrs"]
