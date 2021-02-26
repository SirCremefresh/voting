#!/bin/sh
set -e

#scripts docker infrastructor
chmod +x scripts/docker_compose.sh

#rust infrastructor
rustup override set nightly
rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu

#install diesel_cli
cargo install diesel_cli --no-default-features --features "postgres"

#install auto reload tool
cargo install cargo-watch

make -j watch prepare