#!/bin/sh

#scripts docker infrastructor
chomod +x scripts/docker_compose.sh

#rust infrastructor
rustup override set nightly
rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu

#install diesel_cli
cargo install diesel_cli --no-default-features --features "postgres"
