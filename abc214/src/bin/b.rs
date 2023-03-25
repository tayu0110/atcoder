#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: usize, t: usize}

    let mut res = 0;
    for i in 0..=s {
        for j in 0..=s {
            for k in 0..=s {
                if i + j + k > s {
                    continue;
                }

                if i * j * k <= t {
                    res += 1;
                }
            }
        }
    }

    println!("{}", res);
}
