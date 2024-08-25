#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, p: [(i64, i64, i64); n]}

    for cx in 0..=100 {
        'inner: for cy in 0..=100 {
            let mut h = 0;
            for &(x, y, nh) in &p {
                if nh > 0 {
                    h = nh + (x - cx).abs() + (y - cy).abs();
                    break;
                }
            }

            for &(x, y, nh) in &p {
                if nh > 0 {
                    if h != nh + (x - cx).abs() + (y - cy).abs() {
                        continue 'inner;
                    }
                } else if h > (x - cx).abs() + (y - cy).abs() {
                    continue 'inner;
                }
            }

            println!("{} {} {}", cx, cy, h);
            return;
        }
    }
}
