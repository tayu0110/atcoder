#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, x: [i64; n]}

    let m = x.iter().fold(0, |s, v| s + v.abs());
    let e = (x.iter().fold(0, |s, v| s + v.abs() * v.abs()) as f64).sqrt();
    let c = x.iter().fold(0, |s, v| std::cmp::max(s, v.abs()));

    println!("{}\n{}\n{}", m, e, c);
}
