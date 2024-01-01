use proconio::*;
use static_modint::{Mod998244353, StaticModint};
use std::collections::{HashSet, VecDeque};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {h: usize, w: usize, s: [marker::Chars; h]}

    let mut group = vec![vec![0; w]; h];
    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                continue;
            }

            if group[i][j] > 0 {
                continue;
            }

            cnt += 1;

            let mut nt = VecDeque::new();
            nt.push_back((i, j));
            while let Some((r, c)) = nt.pop_front() {
                if group[r][c] > 0 {
                    continue;
                }
                group[r][c] = cnt;
                for (dy, dx) in vec![(1, 0), (0, 1), (!0, 0), (0, !0)] {
                    let nr = r.wrapping_add(dy);
                    let nc = c.wrapping_add(dx);
                    if nr >= h || nc >= w || s[nr][nc] != '#' || group[nr][nc] > 0 {
                        continue;
                    }

                    nt.push_back((nr, nc));
                }
            }
        }
    }

    let mut set = HashSet::new();
    let mut res = Modint::zero();
    let mut red = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            red += 1;
            set.clear();
            for (dy, dx) in vec![(1, 0), (0, 1), (!0, 0), (0, !0)] {
                let r = i.wrapping_add(dy);
                let c = j.wrapping_add(dx);
                if r < h && c < w && group[r][c] > 0 {
                    set.insert(group[r][c]);
                }
            }

            if set.len() == 0 {
                res += Modint::raw(cnt as u32) + Modint::one();
            } else {
                res += Modint::raw(cnt as u32) - Modint::raw(set.len() as u32 - 1);
            }
        }
    }

    println!("{}", res / Modint::raw(red));
}
