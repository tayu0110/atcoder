#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {s: [String; 3], t: Chars}

    let s = t.into_iter().map(|c| s[c.to_digit(10).unwrap() as usize - 1].clone()).fold("".to_string(), |s, v| { s + v.as_str() });
    println!("{}", s);
}
