use std::collections::HashSet;

use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m], k: usize, cl: [(usize, usize); k], q: usize, q: [(usize, usize); q]}

    let mut uf = UnionFind::new(n);
    for (u, v) in e {
        uf.merge(u - 1, v - 1);
    }

    let mut g = vec![usize::MAX; n];
    for i in 0..n {
        g[i] = uf.root(i);
    }

    let mut bad = HashSet::new();
    for (a, b) in cl {
        bad.insert((g[a - 1], g[b - 1]));
        bad.insert((g[b - 1], g[a - 1]));
    }

    for (x, y) in q {
        if bad.contains(&(g[x - 1], g[y - 1])) {
            println!("No")
        } else {
            println!("Yes")
        }
    }
}
