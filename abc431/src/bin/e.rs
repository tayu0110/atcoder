use std::{collections::VecDeque, fmt::Write as _};

use proconio::*;

fn main() {
    input! {t: usize}

    const D: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

    let mut buf = String::new();
    for _ in 0..t {
        input! {h: usize, w: usize, mut s: [marker::Bytes; h]}
        s.iter_mut().for_each(|s| s.resize(w + 1, b'A'));
        let w = w + 1;

        let mut memo = vec![vec![[u32::MAX; 4]; w]; h];
        memo[0][0][0] = 0;
        let mut nt = VecDeque::new();
        nt.push_back((0, 0, 0));
        while let Some((r, c, dir)) = nt.pop_front() {
            if c == w - 1 {
                continue;
            }
            let t = match s[r][c] {
                b'A' => [dir, (8 - (dir + 1)) % 4, (8 + (5 - dir)) % 4],
                b'B' => [(8 + (5 - dir)) % 4, dir, (8 - (dir + 1)) % 4],
                b'C' => [(8 - (dir + 1)) % 4, dir, (8 + (5 - dir)) % 4],
                _ => unreachable!(),
            };

            let (dr, dc) = D[t[0]];
            let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
            if nr < h && nc < w && memo[r][c][dir] < memo[nr][nc][t[0]] {
                memo[nr][nc][t[0]] = memo[r][c][dir];
                nt.push_front((nr, nc, t[0]));
            }

            for &t in t.iter().skip(1) {
                let (dr, dc) = D[t];
                let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
                if nr < h && nc < w && memo[r][c][dir] + 1 < memo[nr][nc][t] {
                    memo[nr][nc][t] = memo[r][c][dir] + 1;
                    nt.push_back((nr, nc, t));
                }
            }
        }

        writeln!(buf, "{}", memo[h - 1][w - 1].iter().min().unwrap()).unwrap();
    }

    print!("{buf}")
}
