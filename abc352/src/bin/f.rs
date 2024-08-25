use itertools::Itertools;
use proconio::*;
use unionfind::WeightedUnionFind;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, i64); m]}

    let mut uf = WeightedUnionFind::new(n);
    for (a, b, c) in p {
        uf.merge(b - 1, a - 1, c).ok();
    }

    let mut pos = vec![0; n];
    let mut dec = vec![false; n];
    let mut states = vec![];
    let mut lists = vec![];
    for i in 0..n {
        if !dec[i] {
            let mut min = i64::MAX;
            for j in 0..n {
                if uf.is_same(i, j) {
                    pos[j] = uf.diff(i, j);
                    dec[j] = true;
                    min = min.min(pos[j]);
                }
            }

            let mut bits = 0;
            let mut list = vec![];
            for j in 0..n {
                if uf.is_same(i, j) {
                    pos[j] -= min;
                    bits |= 1 << pos[j];
                    list.push((j, pos[j] as usize));
                }
            }
            states.push(bits);
            lists.push(list);
        }
    }

    let mut dp = vec![vec![false; 1 << n]; states.len() + 1];
    let mut reached = vec![vec![false; 1 << n]; states.len() + 1];

    fn rec(
        n: usize,
        now: usize,
        state: usize,
        states: &[usize],
        reached: &mut [Vec<bool>],
        dp: &mut [Vec<bool>],
    ) -> bool {
        if now == states.len() {
            dp[now][state] = true;
            return true;
        }

        if reached[now][state] {
            return dp[now][state];
        }
        reached[now][state] = true;

        let mut f = false;
        for j in 0..n {
            if state & (states[now] << j) != 0 {
                continue;
            }
            let next = state | (states[now] << j);
            if next >= 1 << n {
                break;
            }

            f |= rec(n, now + 1, next, states, reached, dp);
        }

        dp[now][state] = f;
        f
    }

    rec(n, 0, 0, &states, &mut reached, &mut dp);

    let mut res = vec![-1; n];
    for (i, (s, l)) in states.into_iter().zip(lists).enumerate() {
        let mut r = vec![];
        for j in 0..n {
            let mut f = false;
            for k in 0..1 << n {
                if !dp[i][k] {
                    continue;
                }

                if k & (s << j) != 0 {
                    continue;
                }

                let next = k | (s << j);
                if next >= 1 << n {
                    continue;
                }
                f |= dp[i + 1][next];
            }

            if f {
                r.push(j);
            }
        }

        if r.len() > 1 {
            continue;
        }

        let r = r.pop().unwrap();
        for (j, p) in l {
            res[j] = (p + r) as i32 + 1;
        }
    }

    println!("{}", res.iter().join(" "))
}
