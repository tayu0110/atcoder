#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {w: usize, h: usize, n: usize, p: [(usize, usize, u8); n]}

    let (mut u, mut d, mut l, mut r) = (h, 0, 0, w);
    for (x, y, a) in p {
        if a == 1 {
            l = std::cmp::max(l, x)
        } else if a == 2 {
            r = std::cmp::min(r, x)
        } else if a == 3 {
            d = std::cmp::max(d, y)
        } else {
            u = std::cmp::min(u, y)
        }
    }

    if r < l || u < d {
        println!("0")
    } else {
        println!("{}", (r - l) * (u - d))
    }
}
