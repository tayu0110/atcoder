use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();
    a.dedup();
    println!("{}", a.iter().join("\n"))
}
