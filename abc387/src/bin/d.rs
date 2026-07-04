use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'S' {
                let mut res = u32::MAX;
                for v in [false, true] {
                    let mut nt = VecDeque::new();
                    let mut dist = vec![vec![u32::MAX; w]; h];
                    nt.push_back((i, j, v));
                    dist[i][j] = 0;
                    while let Some((r, c, v)) = nt.pop_front() {
                        if v {
                            for (dr, dc) in [(1, 0), (!0, 0)] {
                                let nr = r.wrapping_add(dr);
                                let nc = c.wrapping_add(dc);
                                if nr < h && nc < w && s[nr][nc] != b'#' && dist[nr][nc] == u32::MAX
                                {
                                    dist[nr][nc] = dist[r][c] + 1;
                                    if s[nr][nc] == b'G' {
                                        res = res.min(dist[nr][nc]);
                                    }
                                    nt.push_back((nr, nc, !v));
                                }
                            }
                        } else {
                            for (dr, dc) in [(0, 1), (0, !0)] {
                                let nr = r.wrapping_add(dr);
                                let nc = c.wrapping_add(dc);
                                if nr < h && nc < w && s[nr][nc] != b'#' && dist[nr][nc] == u32::MAX
                                {
                                    dist[nr][nc] = dist[r][c] + 1;
                                    if s[nr][nc] == b'G' {
                                        res = res.min(dist[nr][nc]);
                                    }
                                    nt.push_back((nr, nc, !v));
                                }
                            }
                        }
                    }
                }
                println!("{}", res as i32);
                return;
            }
        }
    }
}
