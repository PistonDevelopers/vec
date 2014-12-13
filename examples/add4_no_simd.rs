extern crate vec;

fn main() {
    println!("ADD4 NO SIMD");
    let mut sum = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _ in range(0u, vec::TEST_ITERATIONS) {
        sum = (sum.0 + 1.0f64, sum.1 + 0.3f64,
                sum.2 + 0.5f64, sum.3 + 0.7f64);
    }
    println!("{}", sum);
}
