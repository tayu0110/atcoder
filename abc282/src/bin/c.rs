#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {_n: usize, mut s: Chars}

    let mut f = false;
    for c in s.iter_mut() {
        if *c == '"' {
            f = !f;
        } else if *c == ',' && !f {
            *c = '.';
        }
    }

    let res = s.into_iter().collect::<String>();
    println!("{}", res);
}
