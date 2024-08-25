use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {h: usize, w: usize, x: usize, mut s: [marker::Chars; h]}

    let mut start = (0, 0);
    let mut goal = (0, 0);
    let mut nt = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i, j)
            } else if s[i][j] == 'G' {
                goal = (i, j)
            } else if s[i][j] == '@' {
                nt.push_back((i, j, 0));
            }
        }
    }

    let mut reached = vec![vec![std::usize::MAX; w]; h];
    while let Some((r, c, nd)) = nt.pop_front() {
        if reached[r][c] != std::usize::MAX {
            continue;
        }
        reached[r][c] = nd;
        s[r][c] = '#';
        if nd == x {
            continue;
        }

        for (dy, dx) in [(0, 1),
            (1, 0),
            (0, 1usize.wrapping_neg()),
            (1usize.wrapping_neg(), 0)] {
            let nr = r.wrapping_add(dy);
            let nc = c.wrapping_add(dx);

            if nr >= h || nc >= w {
                continue;
            }

            if reached[nr][nc] != std::usize::MAX {
                continue;
            }

            if s[nr][nc] == '#' {
                continue;
            }

            nt.push_back((nr, nc, nd + 1));
        }
    }

    let mut nt = VecDeque::new();
    let mut dist = vec![vec![std::usize::MAX; w]; h];
    nt.push_back((start.0, start.1, 0));
    while let Some((r, c, nd)) = nt.pop_front() {
        if dist[r][c] != std::usize::MAX {
            continue;
        }
        dist[r][c] = nd;

        if (r, c) == goal {
            println!("{}", nd);
            return;
        }

        for (dy, dx) in [(0, 1),
            (1, 0),
            (0, 1usize.wrapping_neg()),
            (1usize.wrapping_neg(), 0)] {
            let nr = r.wrapping_add(dy);
            let nc = c.wrapping_add(dx);

            if nr >= h || nc >= w {
                continue;
            }

            if s[nr][nc] == '#' {
                continue;
            }

            if dist[nr][nc] != std::usize::MAX {
                continue;
            }

            nt.push_back((nr, nc, nd + 1));
        }
    }

    println!("-1")
}
