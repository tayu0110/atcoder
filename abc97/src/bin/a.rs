#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: i32, b: i32, c: i32, d: i32}

    if (c - a).abs() <= d {
        println!("Yes")
    } else if (b - a).abs() <= d && (c - b).abs() <= d {
        println!("Yes")
    } else {
        println!("No")
    }
}
