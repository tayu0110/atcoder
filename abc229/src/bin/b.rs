#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {mut a: Chars, mut b: Chars}
    a.reverse();
    b.reverse();

    for (c, d) in a.into_iter().zip(b.into_iter()) {
        if c.to_digit(10).unwrap() + d.to_digit(10).unwrap() > 9 {
            println!("Hard");
            return;
        }
    }

    println!("Easy");
}
