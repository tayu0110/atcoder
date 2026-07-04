use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut a: [u8; n]}
    a.sort_unstable();
    a.dedup();
    println!("{}", a.len());
    println!("{}", a.iter().join(" "))
}
