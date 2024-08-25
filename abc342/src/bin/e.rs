use std::collections::BinaryHeap;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(i64, i64, i64, i64, usize, usize); m]}

    let mut edge = vec![(0, 0, 0, 0); m];
    let mut t = vec![vec![]; n];
    for (i, (l, d, k, c, a, b)) in p.into_iter().enumerate() {
        t[b - 1].push((a - 1, i));
        edge[i] = (l, d, k, c);
    }

    let mut d = vec![-1i64; n];
    let mut nt = BinaryHeap::new();
    for &(to, e) in &t[n - 1] {
        let (l, d, k, _) = edge[e];
        nt.push((l + (k - 1) * d, to));
    }

    while let Some((nd, now)) = nt.pop() {
        if d[now] > nd {
            continue;
        }
        d[now] = nd;
        for &(to, e) in &t[now] {
            if d[to] >= 0 {
                continue;
            }

            let (l, d, k, c) = edge[e];
            let (mut lh, mut rh) = (0, k);
            while rh - lh > 1 {
                let m = (rh + lh) / 2;
                if l + m * d + c <= nd {
                    lh = m;
                } else {
                    rh = m;
                }
            }

            if l + lh * d + c <= nd {
                nt.push((l + lh * d, to));
            }
        }
    }

    for res in d.into_iter().take(n - 1) {
        if res >= 0 {
            println!("{res}")
        } else {
            println!("Unreachable")
        }
    }
}
