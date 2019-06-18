#!/usr/bin/env bash
# build for netlify
cd example
curl https://sh.rustup.rs -sSf | sh -s - --default-toolchain nightly -y
$HOME/.cargo/bin run
