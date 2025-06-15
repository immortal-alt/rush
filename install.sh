#!/bin/bash

# dev tool

cargo build --release --target x86_64-pc-windows-gnu
cargo build --release

sudo mv ./target/release/rush /usr/bin/
