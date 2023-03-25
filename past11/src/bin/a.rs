#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {x: i32, a: i32, b: i32, c: i32}

    let f = b*x + a*b*c - a*x;
    if f < 0 {
        println!("Hare");
    } else if f == 0 {
        println!("Tie");
    } else {
        println!("Tortoise");
    }
}
