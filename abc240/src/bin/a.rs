#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {a: usize, b: usize}
    let (a, b) = (a-1, b-1);

    if (a+1) % 10 == b {
        println!("Yes");
    } else if (b+1) % 10 == a {
        println!("Yes"); 
    } else {
        println!("No");
    }
}
