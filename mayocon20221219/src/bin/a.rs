#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut res = 0;
    for i in 0..n {
        let (x, y) = p[i];
        for j in i + 1..n {
            let (nx, ny) = p[j];
            res = std::cmp::max(res, (nx - x) * (nx - x) + (ny - y) * (ny - y));
        }
    }

    println!("{}", (res as f64).sqrt());
}
