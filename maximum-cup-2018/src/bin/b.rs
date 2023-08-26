use std::collections::VecDeque;

use proconio::*;

fn rot_left(d: (usize, usize)) -> (usize, usize) {
    match d {
        (0, 1) => (1, 0),
        (1, 0) => (0, !0),
        (0, m) if m == !0 => (!0, 0),
        (m, 0) if m == !0 => (0, 1),
        _ => unreachable!(),
    }
}

fn rot_right(d: (usize, usize)) -> (usize, usize) {
    match d {
        (m, 0) if m == !0 => (0, !0),
        (0, 1) => (!0, 0),
        (1, 0) => (0, 1),
        (0, m) if m == !0 => (1, 0),
        _ => unreachable!(),
    }
}

fn main() {
    input! {a: usize, b: usize, h: usize, w: usize, s: [marker::Chars; h]}

    let pos = [(0, 1), (!0, 0), (0, !0), (1, 0)];
    let mut res = vec![vec![vec![vec![vec![false; 4]; b + 1]; a + 1]; w]; h];
    let mut nt = VecDeque::new();
    nt.push_back((1, 1, a, b, 0, 1));
    while let Some((r, c, na, nb, dx, dy)) = nt.pop_front() {
        let p = pos.iter().position(|&v| v == (dx, dy)).unwrap();
        if res[r][c][na][nb][p] {
            continue;
        }
        res[r][c][na][nb][p] = true;
        let (nr, nc) = (r.wrapping_add(dy), c.wrapping_add(dx));
        if nr < h && nc < w && s[nr][nc] != '#' && !res[nr][nc][na][nb][p] {
            nt.push_back((nr, nc, na, nb, dx, dy));
        }

        let (x, y) = rot_left((dx, dy));
        let (nr, nc) = (r.wrapping_add(y), c.wrapping_add(x));
        if nr < h && nc < w && na >= 1 && s[nr][nc] != '#' && !res[nr][nc][na - 1][nb][(p + 3) % 4]
        {
            nt.push_back((nr, nc, na - 1, nb, x, y));
        }

        let (x, y) = rot_right((dx, dy));
        let (nr, nc) = (r.wrapping_add(y), c.wrapping_add(x));
        if nr < h && nc < w && nb >= 1 && s[nr][nc] != '#' && !res[nr][nc][na][nb - 1][(p + 1) % 4]
        {
            nt.push_back((nr, nc, na, nb - 1, x, y));
        }
    }

    if res[h - 2][w - 2][0][0].iter().any(|f| *f) {
        println!("Yes")
    } else {
        println!("No")
    }
}
