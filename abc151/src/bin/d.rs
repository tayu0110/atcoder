use std::collections::VecDeque;

use proconio::*;

fn solve(r: usize, c: usize, s: &Vec<Vec<char>>) -> usize {
    let (h, w) = (s.len(), s[0].len());
    let mut dist = vec![vec![std::usize::MAX; w]; h];
    dist[r][c] = 0;
    let mut nt = VecDeque::new();
    nt.push_back((r, c));
    while let Some((r, c)) = nt.pop_front() {
        for (dx, dy) in vec![(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let nr = r.wrapping_add(dy);
            let nc = c.wrapping_add(dx);

            if nr >= h || nc >= w {
                continue;
            }

            if s[nr][nc] == '#' {
                continue;
            }

            if dist[nr][nc] < std::usize::MAX {
                continue;
            }

            dist[nr][nc] = dist[r][c] + 1;
            nt.push_back((nr, nc));
        }
    }
    *dist
        .iter()
        .flatten()
        .filter(|&&s| s < std::usize::MAX)
        .max()
        .unwrap()
}

fn main() {
    input! {h: usize, w: usize, s: [marker::Chars; h]}

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            res = res.max(solve(i, j, &s));
        }
    }

    println!("{}", res)
}
