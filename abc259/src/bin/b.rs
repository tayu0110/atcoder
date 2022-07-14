#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {a: i64, b: i64, d: usize};

    let mut rad = (b as f64).atan2(a as f64);

    rad += d as f64 * std::f64::consts::PI / 180 as f64;
    let r = ((a * a + b * b) as f64).sqrt();

    println!("{:.10} {:.10}", rad.cos() * r, rad.sin() * r);
}
