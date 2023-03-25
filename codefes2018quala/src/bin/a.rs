#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: usize, b: usize, c: usize, s: usize}

    for a in a..=a + 1 {
        for b in b..=b + 1 {
            for c in c..=c + 1 {
                if a + b + c == s {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No")
}
