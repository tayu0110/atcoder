#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: String, t: String}

    if t.starts_with(s.as_str()) {
        println!("Yes")
    } else {
        println!("No")
    }
}
