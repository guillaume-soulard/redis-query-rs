#!/bin/bash
cargo build --release
cp target/release/redis-query-rs ~/.cargo/bin/rq2