use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut res = vec![0; n];
    for (a, b) in e {
        res[a - 1] += 1;
        res[b - 1] += 1;
    }

    println!("{}", res.iter().join("\n"))
}
