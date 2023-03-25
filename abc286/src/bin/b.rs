#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {_n: usize, s: String}

    let t = s.replace("na", "nya");
    println!("{}", t);
}
