use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Bytes; n]}

    let mut start = (0, 0);
    let mut goal = vec![];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == b'S' {
                start = (i, j);
            } else if s[i][j] == b'G' {
                goal.push((i, j));
            }
        }
    }

    for k in 1..n {
        if goal
            .iter()
            .all(|&(r, c)| start.0.abs_diff(r) % k != 0 || start.1.abs_diff(c) % k != 0)
        {
            println!("-1");
            continue;
        }

        let mut dist = vec![vec![usize::MAX; n]; n];
        let mut nt = BinaryHeap::new();
        nt.push(Reverse((0, start.0, start.1)));
        while let Some(Reverse((nd, r, c))) = nt.pop() {
            if dist[r][c] < usize::MAX {
                continue;
            }
            dist[r][c] = nd;
            for (dy, dx) in [(k, 0), (0, k), (k.wrapping_neg(), 0), (0, k.wrapping_neg())] {
                let nr = r.wrapping_add(dy);
                let nc = c.wrapping_add(dx);

                if nr < n && nc < n && s[nr][nc] != b'X' && dist[nr][nc] == usize::MAX {
                    if r == nr && s[r][c.min(nc)..c.max(nc)].iter().any(|&s| s == b'X') {
                        continue;
                    } else if c == nc && (r.min(nr)..r.max(nr)).any(|r| s[r][c] == b'X') {
                        continue;
                    }
                    nt.push(Reverse((nd + 1, nr, nc)));
                }
            }
        }

        println!(
            "{}",
            goal.iter().map(|&(r, c)| dist[r][c]).min().unwrap() as i64
        )
    }
}
