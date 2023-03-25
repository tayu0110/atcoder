#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, s: Chars}

    if s[n-1] == 'o' {
        println!("Yes");
    } else {
        println!("No");
    }
}
