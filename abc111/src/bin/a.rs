#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: Chars}

    let n = n
        .into_iter()
        .map(|c| {
            if c == '1' {
                '9'
            } else if c == '9' {
                '1'
            } else {
                c
            }
        })
        .collect::<String>();
    println!("{}", n);
}
