use std::cmp::Reverse;

use itertools::Itertools;
use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {h: usize, w: usize, f: [[usize; w]; h], q: usize, query: [(usize, usize, usize, usize, usize, usize); q]}

    let mut height = vec![];
    for i in 0..h {
        for j in 0..w {
            height.push((f[i][j], i * w + j));
        }
    }
    height.sort_unstable_by_key(|&h| Reverse(h));

    let mut bounds = (0..q).map(|i| (i, 0, 1000001)).collect::<Vec<_>>();
    while bounds.iter().any(|&(_, l, r)| r - l > 1) {
        let mut now = 0;
        let mut uf = UnionFind::new(h * w);
        for (i, l, r) in bounds.iter_mut() {
            let m = (*l + *r) / 2;
            let (a, b, y, c, d, z) = query[*i];
            if m > y || m > z {
                *r = m;
                continue;
            }
            while now < height.len() && height[now].0 >= m {
                let (_, j) = height[now];
                now += 1;

                let (r, c) = (j / w, j % w);
                for (dr, dc) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                    let nr = r.wrapping_add(dr);
                    let nc = c.wrapping_add(dc);
                    if nr < h && nc < w && f[nr][nc] >= m {
                        uf.merge(j, nr * w + nc);
                    }
                }
            }

            if uf.is_same((a - 1) * w + b - 1, (c - 1) * w + d - 1) {
                *l = m;
            } else {
                *r = m;
            }
        }

        bounds.sort_unstable_by_key(|v| Reverse((v.1 + v.2) / 2));
    }

    let mut res = vec![0; q];
    for (i, l, r) in bounds {
        let m = (l + r) / 2;
        let (_, _, y, _, _, z) = query[i];
        res[i] = m.abs_diff(y) + m.abs_diff(z);
    }

    println!("{}", res.iter().join("\n"))
}
