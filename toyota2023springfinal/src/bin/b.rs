use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, e: [(usize, usize); n]}

    let mut e = e.into_iter().enumerate().collect::<Vec<_>>();

    e.sort_unstable_by(|(_, (s1, p1)), (_, (s2, p2))| {
        (100 * p2 * s2 + p1 * s1 * p2).cmp(&(100 * p1 * s1 + p2 * s2 * p1))
    });

    println!("{}", e.into_iter().map(|v| v.0 + 1).join(" "))
}
