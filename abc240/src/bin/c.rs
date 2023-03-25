#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, x: usize, p: [(usize, usize); n]}

    let mut memo = std::collections::HashSet::new();
    memo.insert(0);
    for (a, b) in p {
        let mut tmp = std::collections::HashSet::new();
        for &now in &memo {
            tmp.insert(now + a);
            tmp.insert(now + b);
        }

        std::mem::swap(&mut memo, &mut tmp);
    }

    if memo.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
