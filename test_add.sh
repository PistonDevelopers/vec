#!/bin/bash
cargo test
time ./target/add
time ./target/add_no_simd
time ./target/add_simd
time ./target/add_array

