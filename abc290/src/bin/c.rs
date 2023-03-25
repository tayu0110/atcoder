#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}

    a.sort();
    a.dedup();

    let a = a.into_iter().take(k).collect::<Vec<_>>();
    let mut now = 0;
    for a in a {
        if a == now {
            now += 1;
        } else {
            println!("{}", now);
            return;
        }
    }

    println!("{}", now)
}
