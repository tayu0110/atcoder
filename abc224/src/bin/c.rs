#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, mut p: [(i64, i64); n]}
    p.sort();

    let mut res = 0;
    for (i, &(x, y)) in p.iter().enumerate() {
        for (j, &(s, t)) in p.iter().enumerate().skip(i+1) {
            for &(v, w) in p.iter().skip(j+1) {
                if (w - t) * (s - x) != (t - y) * (v - s) {
                    res += 1;
                }
            }
        }
    }

    println!("{}", res);
}
