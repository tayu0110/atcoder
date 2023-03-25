#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {mut a: usize, b: usize, mut c: usize, d: usize}

    while a > 0 && c > 0 {
        if b >= c {
            println!("Yes");
            return;
        }
        c -= b;
        if d >= a {
            println!("No");
            return;
        }
        a -= d;
    }
}
