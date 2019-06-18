#!/usr/bin/env bash
# build for netlify
cd example
curl https://sh.rustup.rs -sSf | sh -s - --default-toolchain nightly -y
source ~/.cargo/env
cargo run
