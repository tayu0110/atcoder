#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {n: usize, x: i64, mut p: [i64; n]}

    for v in p.iter_mut() {
        *v = (*v - x).abs();
    }

    println!("{}", p.into_iter().fold(0, |s, v| gcd(s, v)));
}
