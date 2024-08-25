#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: u128, m: usize}

    const MAX: u128 = 1_000_000_000;
    let mut res = String::new();
    let mut now = 1;
    for _ in 0..m {
        now *= n;

        if now <= MAX {
            res.push('o');
        } else {
            break;
        }
    }

    while res.len() < m {
        res.push('x');
    }

    println!("{}", res);
}
