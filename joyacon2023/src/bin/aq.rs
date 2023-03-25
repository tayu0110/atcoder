use num::Complex;
use proconio::input;

fn main() {
    input! {a: i32, b: i32, d: i32}

    let c = Complex::new(a as f64, b as f64);
    let c = c * Complex::from_polar(&1.0, &(std::f64::consts::PI * d as f64 / 180.0));

    println!("{} {}", c.re, c.im);
}
