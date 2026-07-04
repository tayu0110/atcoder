use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let mut warp = vec![vec![]; 128];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != b'.' && s[i][j] != b'#' {
                warp[s[i][j] as usize].push((i, j));
            }
        }
    }

    let mut nt = VecDeque::new();
    nt.push_back((0usize, 0usize));
    let mut ret = vec![vec![usize::MAX; w]; h];
    ret[0][0] = 0;
    while let Some((r, c)) = nt.pop_front() {
        for (dr, dc) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr < h && nc < w && s[nr][nc] != b'#' && ret[nr][nc] == usize::MAX {
                nt.push_back((nr, nc));
                ret[nr][nc] = ret[r][c] + 1;
            }
        }

        if s[r][c] != b'.' && s[r][c] != b'#' {
            while let Some((nr, nc)) = warp[s[r][c] as usize].pop() {
                if ret[nr][nc] == usize::MAX {
                    ret[nr][nc] = ret[r][c] + 1;
                    nt.push_back((nr, nc));
                }
            }
        }
    }

    println!("{}", ret[h - 1][w - 1] as i64)
}
