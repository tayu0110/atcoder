use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h], a: usize, b: usize, c: usize, d: usize}

    let mut res = vec![vec![usize::MAX; w]; h];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, a - 1, b - 1)));
    while let Some(Reverse((nd, r, c))) = nt.pop() {
        if res[r][c] <= nd {
            continue;
        }
        res[r][c] = nd;

        for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
            let (mut r, mut c) = (r, c);
            let mut kick = false;
            for _ in 0..2 {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if nr >= h || nc >= w {
                    break;
                }

                if s[nr][nc] == b'#' {
                    kick |= true;
                }

                nt.push(Reverse((nd + kick as usize, nr, nc)));
                (r, c) = (nr, nc);
            }
        }
    }

    println!("{}", res[c - 1][d - 1])
}
