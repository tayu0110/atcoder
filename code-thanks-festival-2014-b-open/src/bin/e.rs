use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! { r: usize, c: usize, rs: usize, cs: usize, rg: usize, cg: usize, n: usize, p: [(usize, usize, usize, usize); n] }

    let mut t = vec![vec![false; c + 1]; r + 1];
    for (r, c, h, w) in p {
        for r in r..r + h {
            for c in c..c + w {
                t[r][c] = true;
            }
        }
    }

    let mut reached = vec![vec![false; c + 1]; r + 1];
    let mut nt = VecDeque::new();
    nt.push_back((rs, cs));
    while let Some((y, x)) = nt.pop_front() {
        if reached[y][x] {
            continue;
        }
        reached[y][x] = true;

        if !t[y][x] {
            continue;
        }

        if y == rg && x == cg {
            println!("YES");
            return;
        }

        for (dy, dx) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
            let ny = y.wrapping_add(dy);
            let nx = x.wrapping_add(dx);
            if ny <= r && nx <= c && !reached[ny][nx] {
                nt.push_back((ny, nx));
            }
        }
    }

    println!("NO")
}
