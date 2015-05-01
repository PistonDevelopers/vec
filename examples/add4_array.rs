extern crate vec;

fn main() {
    println!("ADD4 ARRAY");
    let mut sum = [0.0f64; 4];
    for _ in 0 .. vec::TEST_ITERATIONS {
        sum = [
            sum[0] + 1.0f64, 
            sum[1] + 0.3f64, 
            sum[2] + 0.5f64, 
            sum[3] + 0.7f64
        ];
    }
    println!("[{:?}, {:?}, {:?}, {:?}]", sum[0], sum[1], sum[2], sum[3]);
}
