use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}
    let mut q = vec![0; n + 1];
    for (i, p) in p.into_iter().enumerate() {
        q[p] = i + 1;
    }
    println!("{}", q[1..].into_iter().join(" "))
}
