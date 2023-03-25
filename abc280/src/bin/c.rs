#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: Chars, t: Chars}
    let tlen = t.len();

    for (i, (cs, ct)) in s.into_iter().zip(t.into_iter()).enumerate() {
        if cs != ct {
            println!("{}", i + 1);
            return;
        }
    }

    println!("{}", tlen);
}
