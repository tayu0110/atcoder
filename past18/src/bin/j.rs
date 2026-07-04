use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let mut map = 0;
    for i in 0..h {
        for j in 0..w {
            map |= ((s[i][j] == b'#') as usize) << (i * w + j);
        }
    }

    let mut nt = VecDeque::new();
    nt.push_back((0, 0, map));
    let mut memo = vec![vec![vec![usize::MAX; 1 << (h * w)]; w]; h];
    memo[0][0][map] = 0;
    while let Some((r, c, map)) = nt.pop_front() {
        if map == 0 {
            println!("{}", memo[r][c][map]);
            return;
        }

        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr < h && nc < w {
                let next = map ^ (1 << (nr * w + nc));
                if memo[nr][nc][next] == usize::MAX {
                    memo[nr][nc][next] = memo[r][c][map] + 1;
                    nt.push_back((nr, nc, next));
                }
            }
        }
    }
}
