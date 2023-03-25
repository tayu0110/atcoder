#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut s: Chars}

    let mut res = 0usize;
    let mut base = 1usize;
    while let Some(c) = s.pop() {
        res += base * (c as u8 - b'A' + 1) as usize;
        base *= 26;
    }

    println!("{}", res);
}
