use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

const D: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

fn main() {
    input! {h: usize, w: usize, k: usize, p: [(usize, usize); k]}

    let mut memo = vec![vec![i32::MAX; w]; h];
    let mut nt = BinaryHeap::new();
    for (r, c) in p {
        nt.push(Reverse((0, r - 1, c - 1)));
    }

    while let Some(Reverse((nd, r, c))) = nt.pop() {
        if memo[r][c] < i32::MAX {
            continue;
        }
        memo[r][c] = nd;
        for (dr, dc) in D {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr >= h || nc >= w {
                continue;
            }
            if memo[nr][nc] < i32::MAX {
                continue;
            }

            let mut m = [i32::MAX; 4];
            for (i, (dr, dc)) in D.into_iter().enumerate() {
                let nr = nr.wrapping_add(dr);
                let nc = nc.wrapping_add(dc);
                if nr < h && nc < w {
                    m[i] = memo[nr][nc];
                }
            }

            m.sort();
            if m[1] < i32::MAX {
                nt.push(Reverse((m[1] + 1, nr, nc)));
            }
        }
    }

    println!(
        "{}",
        memo.into_iter()
            .flatten()
            .filter_map(|m| (m < i32::MAX).then_some(m as usize))
            .sum::<usize>()
    )
}
