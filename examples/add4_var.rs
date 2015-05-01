extern crate vec;

fn main() {
    println!("ADD VAR");
    let mut sum_x = 0.0f64;
    let mut sum_y = 0.0f64;
    for _ in 0 .. vec::TEST_ITERATIONS {
        sum_x += 1.0f64;
        sum_y += 0.3f64;
    }
    println!("{:?} {:?}", sum_x, sum_y);
}
