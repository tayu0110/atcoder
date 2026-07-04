use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, k: usize, e: [(usize, usize); n - 1]}

        let mut t = vec![vec![]; n];
        for (u, v) in e {
            t[u - 1].push(v - 1);
            t[v - 1].push(u - 1);
        }

        let mut res = vec![usize::MAX; n];
        let mut level = vec![usize::MAX; n];
        let mut par = vec![usize::MAX; n];
        level[0] = 0;
        let mut nt = VecDeque::new();
        nt.push_back(0);
        while let Some(now) = nt.pop_front() {
            for &to in &t[now] {
                if level[to] != usize::MAX {
                    continue;
                }
                par[to] = now;
                level[to] = level[now] + 1;
                nt.push_back(to);
            }
        }

        for i in 0..n {
            if level[i] % k == 0 {
                res[i] = level[i] / k;
            }
        }

        let mut checked = vec![FxHashMap::default(); n];
        for i in 2..k + 2 {
            fn dfs(
                now: usize,
                par: usize,
                k: usize,
                res: &[usize],
                level: &[usize],
                t: &[Vec<usize>],
                checked: &mut [FxHashMap<usize, usize>],
            ) {
                for &to in &t[now] {
                    if to != par && res[to] != usize::MAX {
                        dfs(to, now, k, res, level, t, checked);
                        let keys = checked[to]
                            .keys()
                            .copied()
                            .filter(|&c| level[now] + k <= c)
                            .collect::<Vec<_>>();
                        for c in keys {
                            *checked[now].entry(c).or_insert(0) += 1;
                        }
                    }
                }
            }

            dfs(0, 0, k, &res, &level, &t, &mut checked);

            for j in 0..n {
                if res[j] != usize::MAX {
                    continue;
                }

                let mut now = j;
                for d in 1..=k {
                    now = par[now];
                    if now == usize::MAX {
                        break;
                    }

                    if let Some(&v) = checked[now].get(&(level[now] + k - d)) {
                        if v > 1 {
                            res[j] = i;
                            break;
                        }
                    }
                }
            }
        }

        println!("{}", res.iter().skip(1).map(|&res| res as i64).join(" "));
    }
}
