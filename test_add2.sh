#!/bin/bash
cargo test
time ./target/add2
echo ""
echo ""
time ./target/add2_no_simd
echo ""
echo ""
time ./target/add2_simd
echo ""
echo ""
time ./target/add2_array
echo ""
echo ""
time ./target/add2_var

