#!/bin/bash
cargo build --release
cargo install --path . --bin rq --force