extern crate vec;

use vec::Vec4;

fn main() {
    println!("ADD4");
    let mut sum = Vec4(0.0f32, 0.0, 0.0, 0.0);
    for _ in range(0u, vec::TEST_ITERATIONS) {
        sum = sum.add(&Vec4(1.0f32, 0.3f32, 0.5f32, 0.7f32));
    }
    println!("{}", sum);
}
