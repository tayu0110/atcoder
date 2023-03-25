#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {k: u8}

    let mut s = String::new();
    for i in 0..k {
        s.push((b'A' + i) as char);
    }

    println!("{}", s);
}
