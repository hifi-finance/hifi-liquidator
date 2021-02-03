
FROM rust:1.49 as planner
MAINTAINER Paul Razvan Berg <hello@paulrberg.com>

# 1. Compute the recipe file. We only pay the installation cost once, since it'll be cached
# from the second build onwards.
WORKDIR workspace
RUN cargo install cargo-chef --version 0.1.14
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 2. Cache dependencies.
FROM rust:1.49 as cacher
WORKDIR workspace
RUN cargo install cargo-chef --version 0.1.14
COPY --from=planner /workspace/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# 3. Build the binary.
FROM rust:1.49 as builder
WORKDIR workspace
COPY . .

# Copy over the cached dependencies
COPY --from=cacher /workspace/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# 4. Runtime environment.
FROM rust:1.49 as runtime
WORKDIR workspace
COPY --from=builder /workspace/target/release/hifi-liquidator /usr/local/bin/hifi-liquidator

# Prepare the envrionment for the CLI.
ARG NETWORK="mainnet"
ENV RUST_LOG "hifi_liquidator=trace"
COPY cache/config/${NETWORK}.json config.json
ENTRYPOINT ["/usr/local/bin/hifi-liquidator"]