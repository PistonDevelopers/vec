extern crate vec;

use vec::Vec2;

fn main() {
    println!("ADD2");
    let mut sum = Vec2(0.0f64, 0.0);
    for _ in 0 .. vec::TEST_ITERATIONS {
        sum = sum + Vec2(1.0f64, 0.3f64);
    }
    println!("{:?}", sum);
}
