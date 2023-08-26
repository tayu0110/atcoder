use num::Complex;
use proconio::*;

fn main() {
    input! {n: usize, x: f64, y: f64, s: f64, t: f64}

    let ox = (x + s) / 2.0;
    let oy = (y + t) / 2.0;

    let nx = x - ox;
    let ny = y - oy;

    let rot = Complex::from_polar(&1.0, &(2.0 * std::f64::consts::PI / n as f64));
    let k = Complex::new(nx, ny) * rot;

    println!("{} {}", k.re + ox, k.im + oy);
}
