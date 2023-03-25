#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize}

    let mut m = n.next_power_of_two();
    if m > n {
        m >>= 1;
    }
    println!("{}", m.trailing_zeros());
}
