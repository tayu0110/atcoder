#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, mut l: [usize; n]}
    l.sort();

    if l[0..n - 1].iter().sum::<usize>() > l[n - 1] {
        println!("Yes")
    } else {
        println!("No")
    }
}
