use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    for (i, nei) in t.into_iter().enumerate() {
        println!(
            "{}: {{{}}}",
            i + 1,
            nei.into_iter().map(|i| i + 1).join(", ")
        )
    }
}
