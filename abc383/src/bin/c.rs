use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, d: usize, s: [marker::Bytes; h]}

    let mut nt = VecDeque::new();
    let mut dist = vec![vec![usize::MAX; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'H' {
                nt.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }

    while let Some((r, c)) = nt.pop_front() {
        for (dx, dy) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let nr = r.wrapping_add(dy);
            let nc = c.wrapping_add(dx);
            if nr < h && nc < w && s[nr][nc] == b'.' && dist[nr][nc] > dist[r][c] + 1 {
                dist[nr][nc] = dist[r][c] + 1;
                nt.push_back((nr, nc));
            }
        }
    }

    println!(
        "{}",
        dist.iter().flatten().filter(|&&count| count <= d).count()
    )
}
