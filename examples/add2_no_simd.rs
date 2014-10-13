#![feature(tuple_indexing)]

extern crate vec;

fn main() {
    println!("ADD2 NO SIMD");
    let mut sum = (0.0f64, 0.0f64);
    for _ in range(0u, vec::TEST_ITERATIONS) {
        sum = (sum.0 + 1.0f64, sum.1 + 0.3f64);
    }
    println!("{}", sum);
}
