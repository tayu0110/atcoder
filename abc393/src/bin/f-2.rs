use std::cmp::Reverse;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, a: [usize; n], query: [(usize, usize); q]}

    let mut query = query
        .into_iter()
        .enumerate()
        .map(|(i, (r, x))| (x, r, i))
        .collect::<Vec<_>>();
    query.sort_unstable_by_key(|&(x, _, _)| (Reverse(x)));

    let mut pos = a
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i))
        .collect::<Vec<_>>();
    pos.sort_unstable_by_key(|&(a, i)| (Reverse(a), i));

    let mut lis = vec![usize::MAX; n];
    let mut res = vec![0; q];
    while let Some((x, r, i)) = query.pop() {
        while pos.last().map_or(false, |pos| pos.0 <= x) {
            let (_, p) = pos.pop().unwrap();
            let pos = lis.partition_point(|&l| l < p);
            lis[pos] = lis[pos].min(p);
        }

        res[i] = lis.partition_point(|&l| l < r);
    }

    println!("{}", res.iter().join("\n"));
}
