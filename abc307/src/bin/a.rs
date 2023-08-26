use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; 7*n]}
    println!(
        "{}",
        a.chunks_exact(7).map(|v| v.iter().sum::<usize>()).join(" ")
    );
}
