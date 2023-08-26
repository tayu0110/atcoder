use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: [marker::Chars; n]}

    let mut nt = VecDeque::new();
    for i in 0..4 {
        nt.push_back((1, 1, i));
    }

    const D: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];
    let mut res = vec![vec![vec![false; 4]; m]; n];
    while let Some((r, c, p)) = nt.pop_front() {
        if res[r][c][p] {
            continue;
        }
        res[r][c][p] = true;

        let (nr, nc) = {
            let (dx, dy) = D[p];
            (r.wrapping_add(dy), c.wrapping_add(dx))
        };
        if nr < n && nc < m && s[nr][nc] == '.' {
            nt.push_back((nr, nc, p));
            continue;
        }

        for i in 0..4 {
            if res[r][c][i] {
                continue;
            }

            let (nr, nc) = {
                let (dx, dy) = D[i];
                (r.wrapping_add(dy), c.wrapping_add(dx))
            };
            if nr < n && nc < m && s[nr][nc] == '.' {
                nt.push_back((r, c, i));
            }
        }
    }

    println!(
        "{}",
        res.iter()
            .flatten()
            .filter(|v| v.iter().any(|f| *f))
            .count()
    )
}
