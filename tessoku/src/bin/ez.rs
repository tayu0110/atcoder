use proconio::*;
use std::cmp::Reverse;

fn main() {
    input! {n: usize, mut a: [u16; n]}
    a.sort_unstable_by_key(|&a| Reverse(a));
    println!("{}", a[0] + a[1])
}
