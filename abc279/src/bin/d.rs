#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: i128, b: i128}

    let (mut l, mut r) = (-1, a / b + 10);
    while r - l > 2 {
        let g1 = (r + l * 2) / 3;
        let g2 = (r * 2 + l) / 3;

        let r1 = a as f64 / (g1 as f64).sqrt() + ((g1 - 1) * b) as f64;
        let r2 = a as f64 / (g2 as f64).sqrt() + ((g2 - 1) * b) as f64;

        if r1 > r2 {
            l = g1;
        } else {
            r = g2;
        }
    }

    let mut res = a as f64;
    for g in l.saturating_sub(5)..=r.saturating_add(5) {
        let r = a as f64 / (g as f64).sqrt() + ((g - 1) * b) as f64;
        if r < res {
            res = r;
        }
    }
    println!("{:.10}", res);
}
