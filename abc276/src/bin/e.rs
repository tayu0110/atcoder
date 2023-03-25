#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {h: usize, w: usize, c: [Chars; h]}

    let mut start = (0, 0);
    'base: for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                start = (i, j);
                break 'base;
            }
        }
    }

    let mut d = vec![vec![-1i64; w]; h];
    let mut nt = std::collections::BinaryHeap::new();
    nt.push((0, start.0, start.1));

    while let Some((nd, y, x)) = nt.pop() {
        if (y, x) != start && d[y][x] >= 0 {
            continue;
        }

        if (y, x) == start {
            if nd >= 4 {
                println!("Yes");
                return;
            } else if nd > 0 {
                continue;
            }
        }
        d[y][x] = nd;

        if y > 0 && c[y-1][x] != '#' {
            nt.push((nd+1, y-1, x));
        }
        if y+1 < h && c[y+1][x] != '#' {
            nt.push((nd+1, y+1, x));
        }
        if x > 0 && c[y][x-1] != '#' {
            nt.push((nd+1, y, x-1));
        }
        if x+1 < w && c[y][x+1] != '#' {
            nt.push((nd+1, y, x+1));
        }
    }

    println!("No");
}
