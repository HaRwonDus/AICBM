FROM rust:1.85.1-bookworm AS build
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY examples ./examples
COPY tests ./tests

RUN cargo test --locked && cargo build --release --locked

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=build /app/target/release/aicbm /usr/local/bin/aicbm
COPY examples ./examples

USER 65532:65532
ENTRYPOINT ["aicbm"]
CMD ["validate", "examples/plan.json", "--json"]
