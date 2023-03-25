#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}

    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, 0, 0)));
    let mut dist = vec![vec![std::usize::MAX; w]; h];

    while let Some(std::cmp::Reverse((nd, r, c))) = nt.pop() {
        if dist[r][c] != std::usize::MAX {
            continue;
        }
        dist[r][c] = nd;

        for (dy, dx) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nr = r as i32 + dy;
            let nc = c as i32 + dx;

            if nr < 0 || nc < 0 || nr >= h as i32 || nc >= w as i32 {
                continue;
            }
            if s[nr as usize][nc as usize] == '#' {
                continue;
            }
            if dist[nr as usize][nc as usize] != std::usize::MAX {
                continue;
            }
            nt.push(std::cmp::Reverse((nd, nr as usize, nc as usize)));
        }

        for dy in vec![-2, -1, 0, 1, 2] {
            for dx in vec![-2, -1, 0, 1, 2] {
                if (dy as i32).abs() + (dx as i32).abs() == 4 {
                    continue;
                }
                let nr = r as i32 + dy;
                let nc = c as i32 + dx;

                if nr < 0 || nc < 0 || nr >= h as i32 || nc >= w as i32 {
                    continue;
                }
                if dist[nr as usize][nc as usize] != std::usize::MAX {
                    continue;
                }
                nt.push(std::cmp::Reverse((nd+1, nr as usize, nc as usize)));
            }
        }
    }

    println!("{}", dist[h-1][w-1]);
}
