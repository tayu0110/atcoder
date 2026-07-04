use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, a: [usize; n], b: [usize; m]}

    let mut a = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, i + m))
        .chain(b.into_iter().enumerate().map(|(i, b)| (b, i)))
        .collect::<Vec<_>>();
    a.sort_unstable();

    let mut nt = BinaryHeap::new();
    for (j, &(_, i)) in a.iter().enumerate() {
        if i >= m {
            nt.push((k, j));
        }
    }

    let mut res = vec![0; a.len()];
    while let Some((k, i)) = nt.pop() {
        if res[i] >= k {
            continue;
        }
        res[i] = k;
        if i > 0 {
            let (p, _) = a[i - 1];
            let (na, _) = a[i];
            nt.push((k.saturating_sub(na - p), i - 1));
        }
        if i + 1 < a.len() {
            let (p, _) = a[i + 1];
            let (na, _) = a[i];
            nt.push((k.saturating_sub(p - na), i + 1));
        }
    }

    let mut ans = vec![0; m];
    for (i, (_, j)) in a.into_iter().enumerate() {
        if j < m {
            ans[j] = res[i];
        }
    }

    println!("{}", ans.iter().join("\n"));
}
