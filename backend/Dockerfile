FROM debian:jessie AS builder

RUN apt-get update && apt-get install -y curl libpq-dev build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y --default-toolchain nightly

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder \
  /target/release/voting \
  /usr/local/bin/

WORKDIR /root
CMD ["/usr/local/bin/voting"]