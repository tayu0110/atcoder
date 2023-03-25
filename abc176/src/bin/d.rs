#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {h: usize, w: usize, ch: usize, cw: usize, dh: usize, dw: usize, s: [Chars; h]}

    let mut res = vec![vec![std::usize::MAX; w]; h];
    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, ch - 1, cw - 1)));
    while let Some(std::cmp::Reverse((nd, r, c))) = nt.pop() {
        if res[r][c] != std::usize::MAX {
            continue;
        }
        res[r][c] = nd;

        for dy in -2..=2i32 {
            for dx in -2..=2i32 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                let nr = r as i32 + dy;
                let nc = c as i32 + dx;
                if nr < 0 || nr >= h as i32 || nc < 0 || nc >= w as i32 {
                    continue;
                }
                if s[nr as usize][nc as usize] == '#' {
                    continue;
                }
                if res[nr as usize][nc as usize] != std::usize::MAX {
                    continue;
                }

                if dx.abs() + dy.abs() <= 1 {
                    nt.push(std::cmp::Reverse((nd, nr as usize, nc as usize)));
                } else {
                    nt.push(std::cmp::Reverse((nd + 1, nr as usize, nc as usize)));
                }
            }
        }
    }

    if res[dh - 1][dw - 1] == std::usize::MAX {
        println!("-1")
    } else {
        println!("{}", res[dh - 1][dw - 1])
    }
}
