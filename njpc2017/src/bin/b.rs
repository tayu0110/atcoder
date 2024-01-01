use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {h: usize, w: usize, n: usize, e: [(usize, usize); n]}

    let mut res = e.len() * 4;

    let mut set = e.into_iter().collect::<BTreeSet<_>>();
    while let Some((r, c)) = set.pop_first() {
        if r == 1 {
            res -= 1;
        }
        if r == h {
            res -= 1;
        }
        if c == 1 {
            res -= 1;
        }
        if c == w {
            res -= 1;
        }
        if set.contains(&(r + 1, c)) {
            res -= 1;
        }
        if set.contains(&(r.wrapping_sub(1), c)) {
            res -= 1;
        }
        if set.contains(&(r, c + 1)) {
            res -= 1;
        }
        if set.contains(&(r, c.wrapping_sub(1))) {
            res -= 1;
        }
    }

    println!("{}", (h - 1) * w + (w - 1) * h - res);
}
