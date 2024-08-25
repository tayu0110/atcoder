use std::collections::HashSet;

use proconio::*;

fn search(
    r: usize,
    c: usize,
    h: usize,
    w: usize,
    s: &[Vec<char>],
    memo: &mut HashSet<(usize, usize)>,
) -> usize {
    if memo.contains(&(r, c)) {
        return 0;
    }

    memo.insert((r, c));

    const D: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];
    if D.iter().any(|(dy, dx)| {
        let nr = r.wrapping_add(*dy);
        let nc = c.wrapping_add(*dx);
        nr < h && nc < w && s[nr][nc] == '#'
    }) {
        return 1;
    }

    let mut res = 1;
    for (dy, dx) in D {
        let nr = r.wrapping_add(dy);
        let nc = c.wrapping_add(dx);
        if nr < h && nc < w {
            res += search(nr, nc, h, w, s, memo);
        }
    }

    res
}

fn main() {
    input! {h: usize, w: usize, s: [marker::Chars; h]}

    let mut res = 0;
    let mut t = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' || t.contains(&(i, j)) {
                continue;
            }

            let mut memo = HashSet::new();
            res = res.max(search(i, j, h, w, &s, &mut memo));
            t.extend(memo);
        }
    }

    println!("{res}")
}
