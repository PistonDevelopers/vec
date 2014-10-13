extern crate vec;

fn main() {
    use std::simd::f64x2;
    println!("ADD SIMD");
    let mut sum = f64x2(0.0, 0.0);
    for _ in range(0u, vec::TEST_ITERATIONS) {
        sum = sum + f64x2(1.0, 0.3);
    }
    println!("{}", sum);
}
