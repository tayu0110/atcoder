use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut f = vec![false; n];
    for i in 0..n {
        if !f[i] {
            f[a[i] - 1] = true;
        }
    }

    let res = f
        .into_iter()
        .enumerate()
        .filter(|(_, f)| !f)
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();
    println!("{}", res.len());
    println!("{}", res.into_iter().join(" "))
}
