use std::collections::VecDeque;

use proconio::*;

fn solve(h: usize, w: usize, r: usize, c: usize, mut s: Vec<Vec<u8>>) -> usize {
    s[r][c] = b'.';

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'S' {
                let mut nt = VecDeque::new();
                nt.push_back((i, j));
                let mut d = vec![vec![usize::MAX; w]; h];
                d[i][j] = 0;
                let (mut gr, mut gc) = (usize::MAX, usize::MAX);
                while let Some((r, c)) = nt.pop_front() {
                    for (dr, dc) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
                        let nr = r.wrapping_add(dr);
                        let nc = c.wrapping_add(dc);
                        if nr < h && nc < w && s[nr][nc] != b'#' && d[nr][nc] == usize::MAX {
                            if s[nr][nc] == b'G' {
                                gr = nr;
                                gc = nc;
                            }
                            d[nr][nc] = d[r][c] + 1;
                            nt.push_back((nr, nc));
                        }
                    }
                }

                if gr == usize::MAX {
                    return usize::MAX;
                } else {
                    return d[gr][gc];
                }
            }
        }
    }

    unreachable!()
}

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'#' {
                let r = solve(h, w, i, j, s.clone());
                if r != usize::MAX {
                    res = res.max(r);
                }
            }
        }
    }
    println!("{res}")
}
