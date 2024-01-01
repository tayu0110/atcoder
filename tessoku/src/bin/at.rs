use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, _: [(usize, usize); n]}
    println!("{}\n1", (1..=n).join("\n"));
}
