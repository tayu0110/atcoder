#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {p: [u8; 26]}
    println!("{}", p.into_iter().map(|c| (c - 1 + b'a') as char).collect::<String>());
}
