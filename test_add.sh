#!/bin/bash
cargo test
time ./target/add
echo ""
echo ""
time ./target/add_no_simd
echo ""
echo ""
time ./target/add_simd
echo ""
echo ""
time ./target/add_array
echo ""
echo ""
time ./target/add_var

