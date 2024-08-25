use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, d: usize, a: [usize; n]}

    if d * (k - 1) > n {
        println!("-1");
        return;
    }

    let mut nt = a[..n - d * (k - 1)]
        .iter()
        .enumerate()
        .map(|(i, &a)| Reverse((a, i)))
        .collect::<BinaryHeap<_>>();
    let mut next = n - d * (k - 1);
    let mut prev = usize::MAX;
    let mut res = vec![];
    while let Some(Reverse((na, ind))) = nt.pop() {
        if res.is_empty() || prev + d <= ind {
            prev = ind;
            res.push(na);

            if res.len() == k {
                break;
            }

            nt.extend(
                a[next..n - d * (k - 1 - res.len())]
                    .iter()
                    .enumerate()
                    .map(|(i, &a)| Reverse((a, i + next))),
            );
            next = n - d * (k - 1 - res.len());
        }
    }

    if res.len() == k {
        println!("{}", res.iter().join(" "))
    } else {
        println!("-1")
    }
}
