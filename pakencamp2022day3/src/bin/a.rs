use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {n: usize}
    let m = 2 * n + 1;
    input! {a: [[i64; m]; m]}

    let mut dist = vec![vec![std::i64::MAX; m]; m];
    let mut nt = BinaryHeap::new();
    for i in 0..n {
        nt.push(Reverse((a[0][i], 0, i)));
        nt.push(Reverse((a[m - 1][i], m - 1, i)));
    }
    for i in 1..m - 1 {
        nt.push(Reverse((a[i][0], i, 0)));
    }

    while let Some(Reverse((nd, r, c))) = nt.pop() {
        if dist[r][c] != std::i64::MAX {
            continue;
        }
        dist[r][c] = nd;

        for di in vec![1usize.wrapping_neg(), 0, 1] {
            for dj in vec![1usize.wrapping_neg(), 0, 1] {
                let (ni, nj) = (r.wrapping_add(di), c.wrapping_add(dj));

                if ni < m && nj < m && a[ni][nj] > 0 && dist[ni][nj] == std::i64::MAX {
                    nt.push(Reverse((nd + a[ni][nj], ni, nj)));
                }
            }
        }
    }

    let mut res = std::i64::MAX;
    for i in n + 1..m {
        res = res.min(dist[0][i]);
        res = res.min(dist[m - 1][i]);
    }
    for i in 1..m - 1 {
        res = res.min(dist[i][m - 1]);
    }

    println!("{}", res)
}
