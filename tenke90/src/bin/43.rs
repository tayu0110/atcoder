use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::*;

const INF: i32 = 111222333;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Direction(i32, i32);
const U: Direction = Direction(-1, 0);
const D: Direction = Direction(1, 0);
const L: Direction = Direction(0, -1);
const R: Direction = Direction(0, 1);
impl Direction {
    fn index(&self) -> usize {
        match *self {
            U => 0,
            D => 1,
            L => 2,
            _ => 3
        }
    }
}


fn main() {
    input! {h: usize, w: usize, rs: usize, cs: usize, rt: usize, ct: usize, s: [Chars; h]};

    let (rs, cs) = (rs-1, cs-1);
    let (rt, ct) = (rt-1, ct-1);

    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, rs, cs, U)));
    nt.push(Reverse((0, rs, cs, D)));
    nt.push(Reverse((0, rs, cs, L)));
    nt.push(Reverse((0, rs, cs, R)));

    let mut res = vec![vec![vec![INF; 4]; w]; h];
    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];
    while !nt.is_empty() {
        let Reverse((d, r, c, dir)) = nt.pop().unwrap();
        if std::cmp::min(res[r][c][0], std::cmp::min(res[r][c][1], std::cmp::min(res[r][c][2], res[r][c][3]))) < d {
            continue;
        }
        res[r][c][dir.index()] = d;
        for i in 0..4 {
            let nx = c as i32 - dx[i];
            let ny = r as i32 - dy[i];
            if nx < 0 || nx >= w as i32 || ny < 0 || ny >= h as i32 {
                continue;
            }
            if s[ny as usize][nx as usize] == '#' {
                continue;
            }
            let nd = if dir == Direction(dy[i], dx[i]) {
                d
            } else {
                d + 1
            };
            let ny = ny as usize;
            let nx = nx as usize;
            if std::cmp::min(res[ny][nx][0], std::cmp::min(res[ny][nx][1], std::cmp::min(res[ny][nx][2], res[ny][nx][3]))) < nd {
                continue;
            }
            nt.push(Reverse((nd, ny, nx, Direction(dy[i], dx[i]))));
        }
    }

    // eprintln!("{:?}", res);

    println!("{}", std::cmp::min(res[rt][ct][0], std::cmp::min(res[rt][ct][1], std::cmp::min(res[rt][ct][2], res[rt][ct][3]))));
}