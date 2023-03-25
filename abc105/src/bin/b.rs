#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}
    for i in 0..=25 {
        if i * 4 > n {
            break;
        }

        let j = n - i * 4;
        if j % 7 == 0 {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
