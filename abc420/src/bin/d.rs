use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, a: [marker::Bytes; h]}

    let mut memo = vec![vec![vec![usize::MAX; w]; h]; 2];
    let mut r = 0;
    let mut c = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == b'S' {
                memo[0][i][j] = 0;
                r = i;
                c = j;
            }
        }
    }

    let mut nt = VecDeque::new();
    nt.push_back((r, c, 0));
    while let Some((r, c, state)) = nt.pop_front() {
        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);

            if nr < h && nc < w {
                let next = (state + (a[nr][nc] == b'?') as usize) % 2;
                if memo[next][nr][nc] > memo[state][r][c] + 1
                    && a[nr][nc] != b'#'
                    && a[nr][nc] != [b'x', b'o'][next % 2]
                {
                    memo[next][nr][nc] = memo[state][r][c] + 1;
                    nt.push_back((nr, nc, next));
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == b'G' {
                let ret = memo[0][i][j].min(memo[1][i][j]);
                println!("{}", ret as i64)
            }
        }
    }
}
