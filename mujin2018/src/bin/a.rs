#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: String}

    if s.starts_with("MUJIN") {
        println!("Yes")
    } else {
        println!("No")
    }
}
