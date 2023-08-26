use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; 3*n]}

    let mut found = vec![false; n + 1];
    let mut idx = vec![std::usize::MAX; n + 1];
    for (i, &a) in a.iter().enumerate() {
        if !found[a] {
            found[a] = true;
            continue;
        }

        if idx[a] != std::usize::MAX {
            continue;
        }

        idx[a] = i;
    }

    let mut v = idx
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<Vec<_>>();
    v.sort();
    let mut res = vec![];
    for (_, r) in v.into_iter().take(n) {
        res.push(r);
    }
    println!("{}", res.iter().join(" "))
}
