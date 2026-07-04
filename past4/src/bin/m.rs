use itertools::Itertools;
use proconio::*;
use unionfind::UnionFind;

fn dfs(now: usize, level: &mut [usize], par: &mut [(usize, usize)], t: &[Vec<(usize, usize)>]) {
    for &(to, index) in &t[now] {
        if to != par[now].0 {
            par[to] = (now, index);
            level[to] = level[now] + 1;
            dfs(to, level, par, t);
        }
    }
}

fn main() {
    input! {n: usize, q: usize, e: [(usize, usize); n - 1], mut query: [(usize, usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (i, (a, b)) in e.into_iter().enumerate() {
        t[a - 1].push((b - 1, i));
        t[b - 1].push((a - 1, i));
    }

    let mut level = vec![0; n];
    let mut par = vec![(usize::MAX, 0); n];
    dfs(0, &mut level, &mut par, &t);

    let mut uf = UnionFind::new(n);
    let mut high = (0..n).collect::<Vec<_>>();
    let mut res = vec![0; n - 1];
    while let Some((u, v, c)) = query.pop() {
        let (mut u, mut v) = (u - 1, v - 1);
        while u != v {
            if level[u] < level[v] {
                (u, v) = (v, u);
            }

            if high[uf.root(u)] == u {
                let (par, index) = par[u];
                if par != usize::MAX {
                    res[index] = c;
                    let new = high[uf.root(par)];
                    high[uf.root(u)] = new;
                    uf.merge(u, par);
                }
            } else {
                u = high[uf.root(u)];
            }
        }
    }

    println!("{}", res.iter().join("\n"))
}
