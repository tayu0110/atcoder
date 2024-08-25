use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, mut s: [marker::Chars; n]}

    let mut pos = [(usize::MAX, usize::MAX); 2];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'P' {
                if pos[0] != (usize::MAX, usize::MAX) {
                    pos[1] = (i, j);
                } else {
                    pos[0] = (i, j);
                }
                s[i][j] = '.';
            }
        }
    }

    let mut memo = vec![vec![vec![vec![usize::MAX; n]; n]; n]; n];
    let (r0, c0) = pos[0];
    let (r1, c1) = pos[1];
    memo[pos[0].0][pos[0].1][pos[1].0][pos[1].1] = 0;
    let mut nt = VecDeque::new();
    nt.push_back((r0, c0, r1, c1));
    while let Some((r, c, y, x)) = nt.pop_front() {
        for (dy, dx) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let (mut nr, mut nc) = (r, c);
            let (mut ny, mut nx) = (y, x);
            for (dy, dx, r, c) in [(dy, dx, &mut nr, &mut nc), (dy, dx, &mut ny, &mut nx)] {
                let nr = (*r).wrapping_add(dy);
                let nc = (*c).wrapping_add(dx);

                if nr < n && nc < n && s[nr][nc] == '.' {
                    *r = nr;
                    *c = nc;
                }
            }

            if nr == ny && nc == nx {
                println!("{}", memo[r][c][y][x] + 1);
                return;
            }

            if memo[nr][nc][ny][nx] == usize::MAX {
                memo[nr][nc][ny][nx] = memo[r][c][y][x] + 1;
                nt.push_back((nr, nc, ny, nx));
            }
        }
    }

    println!("-1");
}
