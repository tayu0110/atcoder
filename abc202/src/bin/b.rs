#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut s: Chars}

    s.reverse();

    s.iter_mut().for_each(|c| {
        *c = match *c {
            '0' => '0',
            '1' => '1',
            '6' => '9',
            '8' => '8',
            '9' => '6',
            _ => *c,
        }
    });

    println!("{}", s.into_iter().collect::<String>())
}
