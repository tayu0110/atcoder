use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut l: [usize; n]}

    l.sort_unstable_by_key(|l| Reverse(*l));

    println!("{}", l[..k].iter().sum::<usize>())
}
