use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, mut s: [marker::Bytes; h]}

    let mut nt = VecDeque::new();
    let mut dist = vec![vec![usize::MAX; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'E' {
                nt.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }

    while let Some((r, c)) = nt.pop_front() {
        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr >= h || nc >= w {
                continue;
            }

            if dist[nr][nc] < usize::MAX || s[nr][nc] == b'#' {
                continue;
            }

            dist[nr][nc] = dist[r][c] + 1;
            nt.push_back((nr, nc));
            match (dr, dc) {
                (0, 1) => s[nr][nc] = b'<',
                (1, 0) => s[nr][nc] = b'^',
                (0, usize::MAX) => s[nr][nc] = b'>',
                (usize::MAX, 0) => s[nr][nc] = b'v',
                _ => {}
            }
        }
    }

    for s in s {
        println!("{}", s.into_iter().map(|c| c as char).collect::<String>());
    }
}
