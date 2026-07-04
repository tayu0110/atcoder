use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {h: usize, w: usize, x: usize, p: usize, q: usize, mut s: [[usize; w]; h]}

    let mut now = s[p - 1][q - 1];
    s[p - 1][q - 1] = usize::MAX;
    let mut nt = BinaryHeap::new();
    for (dy, dx) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
        let np = (p - 1).wrapping_add(dy);
        let nq = (q - 1).wrapping_add(dx);
        if np < h && nq < w {
            nt.push(Reverse((s[np][nq], np, nq)));
            s[np][nq] = usize::MAX;
        }
    }

    while let Some(Reverse((ps, r, c))) = nt.pop() {
        if now <= ps.saturating_mul(x) {
            break;
        }
        now += ps;

        for (dy, dx) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let nr = r.wrapping_add(dy);
            let nc = c.wrapping_add(dx);
            if nr < h && nc < w && s[nr][nc] != usize::MAX {
                nt.push(Reverse((s[nr][nc], nr, nc)));
                s[nr][nc] = usize::MAX;
            }
        }
    }

    println!("{now}")
}
