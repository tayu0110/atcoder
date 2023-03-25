#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: Chars}

    for i in 1..n {
        for l in (0..n).take_while(|v| *v + i <= n) {
            if l + i == n {
                println!("{}", l);
                break;
            }
            if s[l] == s[l + i] {
                println!("{}", l);
                break;
            }
        }
    }
}
