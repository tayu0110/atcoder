#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize}

    let mut res = 0;
    for i in (1..=m).take_while(|&i| i * i <= m) {
        if m % i == 0 {
            if m / i >= n {
                res = std::cmp::max(res, i);
            }
            if i >= n {
                res = std::cmp::max(res, m / i);
            }
        }
    }

    println!("{}", res);
}
