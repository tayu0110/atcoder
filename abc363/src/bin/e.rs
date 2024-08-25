use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {h: usize, w: usize, y: usize, a: [[usize; w]; h]}

    let mut rem = vec![vec![false; w]; h];

    let mut next = BinaryHeap::new();
    for i in 0..h {
        next.push(Reverse((a[i][0], i, 0)));
        next.push(Reverse((a[i][w - 1], i, w - 1)));
    }
    for j in 1..w - 1 {
        next.push(Reverse((a[0][j], 0, j)));
        next.push(Reverse((a[h - 1][j], h - 1, j)));
    }

    let mut res = h * w;
    for i in 1..=y {
        while let Some(&Reverse((height, r, c))) = next.peek() {
            if rem[r][c] {
                next.pop();
                continue;
            }

            if height > i {
                break;
            }

            next.pop();
            rem[r][c] = true;
            res -= 1;

            for (dy, dx) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                let nr = r.wrapping_add(dy);
                let nc = c.wrapping_add(dx);

                if nr < h && nc < w && !rem[nr][nc] {
                    next.push(Reverse((a[nr][nc], nr, nc)));
                }
            }
        }

        println!("{res}");
    }
}
