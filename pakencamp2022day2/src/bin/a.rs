use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut out = vec![0; n];
    for (a, _) in p {
        out[a - 1] += 1;
    }

    let res = out
        .into_iter()
        .enumerate()
        .filter(|(_, v)| *v == 0)
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();
    println!("{}", res.len());
    println!("{}", res.into_iter().join(" "))
}
