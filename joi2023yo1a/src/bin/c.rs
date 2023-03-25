#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {_n: usize, s: Chars}
    let mut now = 0u8;
    let mut res = 0;
    for c in s {
        if c == 'L' {
            now = now.saturating_sub(1);
        } else {
            now = std::cmp::min(now + 1, 2);
            if now == 2 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
