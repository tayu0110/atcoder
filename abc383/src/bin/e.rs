use std::cmp::Reverse;

use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, k: usize, mut e: [(usize, usize, usize); m], a: [usize; k], b: [usize; k]}

    e.sort_unstable_by_key(|e| Reverse(e.2));
    let mut t = vec![(0, u8::MAX); n];
    for a in a {
        t[a - 1].1 = 0;
        t[a - 1].0 += 1;
    }
    for b in b {
        t[b - 1].1 = 1;
        t[b - 1].0 += 1;
    }

    let mut res = 0;
    let mut uf = UnionFind::new(n);
    while let Some((u, v, w)) = e.pop().map(|(u, v, w)| (u - 1, v - 1, w)) {
        if uf.is_same(u, v) {
            continue;
        }

        let ru = uf.root(u);
        let rv = uf.root(v);
        uf.merge(u, v);
        let new = uf.root(u);
        if t[ru].1 < u8::MAX && t[rv].1 < u8::MAX {
            if t[ru].1 == t[rv].1 {
                t[new] = (t[ru].0 + t[rv].0, t[ru].1);
            } else {
                res += w * t[ru].0.min(t[rv].0);
                if t[ru].0 == t[rv].0 {
                    t[new] = (0, u8::MAX);
                } else {
                    t[new] = (
                        t[ru].0.abs_diff(t[rv].0),
                        if t[ru].0 < t[rv].0 { t[rv].1 } else { t[ru].1 },
                    );
                }
            }
        } else {
            if t[ru].1 < u8::MAX {
                t[new] = t[ru];
            } else {
                t[new] = t[rv];
            }
        }
        if new != ru {
            t[ru] = (0, u8::MAX);
        }
        if new != rv {
            t[rv] = (0, u8::MAX);
        }
    }

    println!("{res}")
}
