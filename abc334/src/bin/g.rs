use graph::{low_link, UnDirected};
use proconio::*;
use static_modint::{Mod998244353, StaticModint};
use std::collections::{BTreeMap, VecDeque};

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
                for (dy, dx) in [(1, 0), (0, 1), (!0, 0), (0, !0)] {
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

    let mut group_member = vec![vec![]; cnt + 1];
    let mut res = Modint::zero();
    let mut green = 0;
    for i in 0..h {
        for j in 0..w {
            if group[i][j] == 0 {
                continue;
            }

            green += 1;
            group_member[group[i][j]].push(i * w + j);
            res += Modint::raw(cnt as u32);
        }
    }

    for member in group_member {
        if member.len() == 1 {
            let k = member[0];
            let (_r, _c) = (k / w, k % w);
            res -= Modint::one();
            continue;
        }

        let mut map = BTreeMap::new();
        for &m in &member {
            map.insert(m, 0);
        }
        let mut rev = vec![0; map.len()];
        let mut cnt = 0;
        for (k, v) in map.iter_mut() {
            *v = cnt;
            rev[cnt] = *k;
            cnt += 1;
        }

        let mut graph = vec![vec![]; cnt];
        for m in member {
            let (r, c) = (m / w, m % w);
            for (dx, dy) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                let nr = r.wrapping_add(dy);
                let nc = c.wrapping_add(dx);
                if nr < h && nc < w && group[r][c] == group[nr][nc] {
                    graph[*map.get(&m).unwrap()].push(*map.get(&(nr * w + nc)).unwrap());
                }
            }
        }

        let list = low_link::<UnDirected>(&graph.into());
        res += Modint::raw(list.len() as u32);
        let mut c = vec![0; cnt];

        for &(s, t) in &list {
            c[s] += 1;
            c[t] += 1;
        }

        for (s, t) in list {
            if c[s] == 1 && c[t] == 1 {
                res += Modint::one();
            }
        }
        for c in c {
            if c > 1 {
                res += Modint::raw(c - 1);
            }
        }
    }

    eprintln!("res: {res:?}");

    println!("{}", res / Modint::raw(green))
}
