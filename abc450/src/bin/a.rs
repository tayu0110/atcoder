use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize}
    println!("{}", (1..=n).rev().join(","))
}
