#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn main() {
    input! {n: usize, a: [usize; n]}
    println!("{}", a.into_iter().fold(0, gcd))
}
