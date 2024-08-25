#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, ax: usize, ay: usize, bx: usize, by: usize, s: [Chars; n]}

    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, ax-1, ay-1)));
    let mut dist = vec![vec![std::usize::MAX; n]; n];
    let mut reached = vec![vec![false; n]; n];
    while let Some(std::cmp::Reverse((nd, x, y))) = nt.pop() {
        if reached[x][y] {
            continue;
        }
        dist[x][y] = nd;
        reached[x][y] = true;

        for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let (mut nx, mut ny) = (x as i32, y as i32);
            loop {
                nx += dx;
                ny += dy;
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n as i32 {
                    break;
                }
                if s[nx as usize][ny as usize] == '#' {
                    break;
                }
                if dist[nx as usize][ny as usize] < nd + 1 {
                    break;
                }
                if dist[nx as usize][ny as usize] > nd + 1 {
                    dist[nx as usize][ny as usize] = nd + 1;
                    nt.push(std::cmp::Reverse((nd + 1, nx as usize, ny as usize)));
                }
            }
        }
    }

    if dist[bx-1][by-1] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[bx-1][by-1]);
    }
}
