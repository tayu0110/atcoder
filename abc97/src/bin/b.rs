#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {x: usize}

    let mut res = 0;
    for i in 1..=x {
        let mut now = i * i;

        if now > x {
            continue;
        }

        for _ in 0..10 {
            if now * i <= x {
                now *= i;
            } else {
                break;
            }
        }

        res = std::cmp::max(res, now);
    }

    println!("{}", res)
}
