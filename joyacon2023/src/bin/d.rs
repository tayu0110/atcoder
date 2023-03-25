#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: [usize; n]}
    let res: usize = a.into_iter().sum();
    println!("{}", res);
}
