#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: String}

    let t = s[0..n / 2].to_string().repeat(2);
    if t == s {
        println!("Yes");
    } else {
        println!("No");
    }
}
