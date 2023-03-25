#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize, mut x: usize};

    if a * a * b <= 2 * x {
        x = a * a * b - x;
        let h = 2.0 * x as f64 / (a * a) as f64;
        println!("{}", f64::atan2(h, a as f64) * 180.0 / std::f64::consts::PI);
    } else {
        let w = 2.0 * x as f64 / (a * b) as f64;
        println!("{}", f64::atan2(b as f64, w) * 180.0 / std::f64::consts::PI);
    }

}
