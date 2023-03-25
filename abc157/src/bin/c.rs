#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, p: [(usize, u8); m]}

    for i in 0..1000usize {
        let k = i.to_string();

        if k.len() != n {
            continue;
        }

        let mut bad = false;
        let k = k.bytes().collect::<Vec<_>>();
        for &(s, c) in &p {
            bad |= k[s - 1] - b'0' != c;
        }

        if !bad {
            println!("{}", i);
            return;
        }
    }

    println!("-1")
}
