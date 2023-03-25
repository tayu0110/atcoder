#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut n: Chars}
    n.reverse();
    let len = n.len();

    for _ in 0..len {
        let mut k = n.clone();
        k.reverse();
        if n == k {
            println!("Yes");
            return;
        }
        n.push('0');
    }

    println!("No");
}
