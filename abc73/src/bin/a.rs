#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: Chars}

    if n.into_iter().any(|c| c == '9') {
        println!("Yes")
    } else {
        println!("No")
    }
}
