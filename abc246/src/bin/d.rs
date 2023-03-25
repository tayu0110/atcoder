#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: i128}

    let f = |a, b| a * a * a + a * a * b + a * b * b + b * b * b;
    let mut min = std::i128::MAX;
    for i in (0..=n).take_while(|i| *i * *i * *i <= n) {
        let (mut l, mut r) = (-1, 10_000_000);
        while r - l > 1 {
            let m = (r + l) / 2;
            let x = f(i, m);
            if x >= n {
                r = m;
            } else {
                l = m;
            }
        }

        min = std::cmp::min(min, f(i, r));
    }

    println!("{}", min);
}
