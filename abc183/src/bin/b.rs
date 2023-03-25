#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {sx: i64, sy: i64, gx: i64, gy: i64}

    println!("{}", sx as f64 - (sy * (gx - sx)) as f64 / (-gy - sy) as f64);
}
