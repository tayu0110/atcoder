use proconio::*;
use std::{cmp::Reverse, collections::BinaryHeap};

fn solve(h: usize, w: usize, r: usize, c: usize, a: &[Vec<usize>]) -> usize {
    let mut res = vec![vec![usize::MAX; w]; h];

    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, r, c)));
    while let Some(Reverse((nd, r, c))) = nt.pop() {
        if res[r][c] < usize::MAX {
            continue;
        }
        res[r][c] = nd;

        for (dy, dx) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let (nr, nc) = (r.wrapping_add(dy), c.wrapping_add(dx));
            if nr < h && nc < w && res[nr][nc] == usize::MAX {
                nt.push(Reverse((nd + a[nr][nc], nr, nc)));
            }
        }
    }

    res[h - 1][0] + res[h - 1][w - 1] + res[0][w - 1] + a[r][c]
}

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]}

    let mut res = usize::MAX;
    for r in 0..h {
        for c in 0..w {
            res = res.min(solve(h, w, r, c, &a));
        }
    }

    println!("{res}")
}
