extern crate vec;

fn main() {
    use std::simd::f32x4;
    println!("ADD4 SIMD");
    let mut sum = f32x4(0.0, 0.0, 0.0, 0.0);
    for _ in range(0u, vec::TEST_ITERATIONS) {
        sum = sum + f32x4(1.0, 0.3, 0.5, 0.7);
    }
    println!("{}", sum);
}
