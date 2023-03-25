#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: Chars, k: usize}

    let n = s.len();
    let mut t = vec![];

    for i in 0..n {
        let mut v = String::new();
        for j in i..std::cmp::min(i + 10, n) {
            v.push(s[j]);

            t.push(v.clone())
        }
    }

    t.sort();
    t.dedup();

    println!("{}", t[k - 1])
}
