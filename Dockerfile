FROM rust:1.49

WORKDIR /usr/src/liquidator
COPY . .

RUN cargo install --path ./liquidator

ENV RUST_LOG "hifi_liquidator=trace"
ENTRYPOINT ["hifi-liquidator"]