extern crate vec;

fn main() {
    println!("ADD2 ARRAY");
    let mut sum = [0.0f64; 2];
    for _ in 0 .. vec::TEST_ITERATIONS {
        sum = [sum[0] + 1.0f64, sum[1] + 0.3f64];
    }
    println!("[{:?}, {:?}]", sum[0], sum[1]);
}
