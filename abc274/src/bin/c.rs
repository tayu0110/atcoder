#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, mut a: [usize; n]}

    let mut memo = vec![std::usize::MAX; 2 * n + 2];
    memo[1] = 0;

    for i in 1..=2*n+1 {
        if memo[i] != std::usize::MAX {
            println!("{}", memo[i]);
            continue;
        }

        let par = a[i / 2-1];
        memo[i] = memo[par] + 1;
        println!("{}", memo[i]);
    }
}
