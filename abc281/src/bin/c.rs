#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, t: usize, a: [usize; n]}

    let sum: usize = a.iter().sum();

    let t = t % sum;

    let mut now = 0;

    for i in 0..n {
        if now <= t && t < now + a[i] {
            println!("{} {}", i + 1, t - now);
            return;
        }

        now += a[i];
    }
}
