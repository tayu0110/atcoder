#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {_: usize, s: Chars}

    let mut now = 0;
    let mut res = 0i32;
    for c in s {
        if c == 'I' {
            now += 1;
        } else {
            now -= 1;
        }

        res = std::cmp::max(res, now)
    }

    println!("{}", res)
}
