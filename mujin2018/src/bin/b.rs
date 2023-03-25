#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut a: u8, s: Chars}

    if a == 0 {
        println!("Yes");
        return;
    }

    for c in s {
        if c == '+' {
            a += 1;
        } else {
            a -= 1;
        }

        if a == 0 {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
