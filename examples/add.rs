extern crate vec;

use vec::Vec2;

fn main() {
    println!("ADD");
    let mut sum = Vec2(0.0f64, 0.0);
    for _ in range(0u, vec::TEST_ITERATIONS) {
        sum = sum.add(&Vec2(1.0f64, 0.3f64));
    }
    println!("{}", sum);
}
