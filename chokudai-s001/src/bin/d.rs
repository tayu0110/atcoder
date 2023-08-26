use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();
    println!("{}", a.iter().join(" "))
}
