use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut s: [String; n]}
    s.sort_unstable_by_key(|s| s.len());
    println!("{}", s.into_iter().join(""))
}
