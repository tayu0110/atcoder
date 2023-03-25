#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize}

    let mut v = vec![0; m];
    for _ in 0..n {
        input! {k: usize, a: [usize; k]}
        for a in a {
            v[a - 1] += 1;
        }
    }

    println!("{}", v.into_iter().filter(|v| *v == n).count());
}
