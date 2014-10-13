#!/bin/bash
cargo test
time ./target/add4
echo ""
echo ""
time ./target/add4_no_simd
echo ""
echo ""
time ./target/add4_simd
echo ""
echo ""
time ./target/add4_array
echo ""
echo ""
time ./target/add4_var

