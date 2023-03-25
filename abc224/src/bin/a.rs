#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {mut s: Chars}
    s.reverse();
    let s = s.into_iter().collect::<String>();
    if &s[0..2] == "re" {
        println!("er");
    } else {
        println!("ist");
    }
}
