use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Chars; h]}

    let mut dist = vec![vec![std::usize::MAX; w]; h];
    let mut nt = VecDeque::new();
    nt.push_back((0usize, 0usize));
    dist[0][0] = 0;
    while let Some((r, c)) = nt.pop_front() {
        for (dx, dy) in vec![(0usize, 1usize), (1, 0), (0, !0), (!0, 0)] {
            let (nr, nc) = (r.wrapping_add(dy), c.wrapping_add(dx));
            if nr < h && nc < w && s[r][c] != s[nr][nc] && dist[nr][nc] == std::usize::MAX {
                dist[nr][nc] = dist[r][c] + 1;
                nt.push_back((nr, nc));
            }
        }
    }

    if dist[h - 1][w - 1] < std::usize::MAX {
        println!("{}", dist[h - 1][w - 1])
    } else {
        println!("-1")
    }
}
