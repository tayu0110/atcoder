#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {_n: usize, s: Chars}

    let c = s.into_iter().take_while(|c| *c == '0').count();

    if c % 2 == 1 {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}
