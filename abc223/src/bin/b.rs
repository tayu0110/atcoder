#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {mut s: Chars}

    let n = s.len();
    let mut max = s.iter().collect::<String>();
    let mut min = s.iter().collect::<String>();

    for _ in 0..n {
        s.rotate_left(1);
        let ns = s.iter().collect::<String>();
        max = std::cmp::max(max, ns.clone());
        min = std::cmp::min(min, ns);
    }

    println!("{}", min);
    println!("{}", max);
}
