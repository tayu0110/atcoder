#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {k: usize, s: Chars}

    if s.len() <= k {
        println!("{}", s.into_iter().collect::<String>())
    } else {
        println!(
            "{}",
            s.into_iter()
                .take(k)
                .chain("...".chars())
                .collect::<String>()
        )
    }
}
