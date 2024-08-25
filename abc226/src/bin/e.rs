use proconio::*;
use static_modint::{Mod998244353, StaticModint};
use std::collections::HashSet;
use unionfind::UnionFind;

type Modint = StaticModint<Mod998244353>;

fn cycle_detect(now: usize, prev: usize, used: &mut [bool], t: &Vec<Vec<usize>>) -> bool {
    if used[now] {
        return true;
    }

    used[now] = true;
    for &to in &t[now] {
        if to == prev {
            continue;
        }

        let f = cycle_detect(to, now, used, t);
        if f {
            return f;
        }
    }

    used[now] = false;
    false
}

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    if n != m {
        println!("0");
        return;
    }

    let mut t = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for (u, v) in p {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
        uf.merge(u - 1, v - 1);
    }

    if t.iter().any(|v| v.is_empty()) {
        println!("0");
        return;
    }

    let mut set = HashSet::new();
    let mut used = vec![false; n];
    for i in 0..n {
        if !set.contains(&uf.root(i)) && !cycle_detect(i, i, &mut used, &t) {
            println!("0");
            return;
        }
        set.insert(uf.root(i));
    }

    println!("{}", Modint::raw(2).pow(set.len() as u64))
}
