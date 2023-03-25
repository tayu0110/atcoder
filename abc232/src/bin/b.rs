#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {mut s: Bytes, mut t: Bytes}
    t.iter_mut().for_each(|c| *c -= b'a');
    s.iter_mut().for_each(|c| *c -= b'a');

    for _ in 0..26 {
        if s == t {
            println!("Yes");
            return;
        }

        s.iter_mut().for_each(|c| *c = (*c + 1) % 26);
    }

    println!("No");
}
