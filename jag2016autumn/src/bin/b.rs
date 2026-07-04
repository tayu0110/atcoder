use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let (mut si, mut sj) = (0, 0);
    let (mut gi, mut gj) = (0, 0);
    let mut dist = vec![vec![u16::MAX; w]; h];
    let mut nt = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'@' {
                (si, sj) = (i, j);
            } else if s[i][j] == b'$' {
                dist[i][j] = 0;
                nt.push_back((i, j));
            } else if s[i][j] == b'%' {
                (gi, gj) = (i, j);
            }
        }
    }

    while let Some((r, c)) = nt.pop_front() {
        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr < h && nc < w && s[nr][nc] != b'#' && dist[r][c] + 1 < dist[nr][nc] {
                dist[nr][nc] = dist[r][c] + 1;
                nt.push_back((nr, nc));
            }
        }
    }

    nt.push_back((si, sj));
    dist[si][sj] = 0;
    while let Some((r, c)) = nt.pop_front() {
        if (r, c) == (gi, gj) {
            println!("Yes");
            return;
        }
        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr < h && nc < w && s[nr][nc] != b'#' && dist[r][c] + 1 < dist[nr][nc] {
                dist[nr][nc] = dist[r][c] + 1;
                nt.push_back((nr, nc));
            }
        }
    }

    println!("No")
}
