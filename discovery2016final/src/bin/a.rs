use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, d: [usize; n]}

    let mut res = vec![0; n];
    let prise = [100_000, 50_000, 30_000, 20_000, 10_000];
    for (i, d) in d.into_iter().enumerate() {
        res[d - 1] = *prise.get(i).unwrap_or(&0);
    }
    println!("{}", res.into_iter().join("\n"))
}
