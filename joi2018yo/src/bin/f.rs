use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, a: [(usize, usize, usize, usize); n]}

    let mut t = vec![vec![vec![]; n]; n];
    for (a, b, c, d) in a {
        t[a][b].push((c, d));
        t[c][d].push((a, b));
    }

    let mut memo = vec![vec![0; n]; n];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0usize, 0, 0)));
    while let Some(Reverse((d, r, c))) = nt.pop() {
        if memo[r][c] < usize::MAX {
            continue;
        }
        memo[r][c] = d;

        for &(nr, nc) in &t[r][c] {
            if memo[nr][nc] == usize::MAX {
                nt.push(Reverse((d + 1, nr, nc)));
            }
        }
    }
}
