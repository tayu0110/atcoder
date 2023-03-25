#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: String}

    let v = s.split("/").collect::<Vec<_>>();

    if v[1].parse::<u32>().unwrap() <= 4u32 {
        println!("Heisei")
    } else {
        println!("TBD")
    }
}
