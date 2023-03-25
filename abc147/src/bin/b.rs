#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {s: Chars}
    let mut t = s.clone();
    t.reverse();

    let mut res = 0;
    for (c, d) in s.into_iter().zip(t.into_iter()) {
        if c != d {
            res += 1;
        }
    }

    println!("{}", res / 2);
}
