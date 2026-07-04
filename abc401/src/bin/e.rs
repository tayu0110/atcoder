use proconio::*;
use rustc_hash::FxHashSet;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for &(u, v) in &e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut map = FxHashSet::default();
    let mut uf = UnionFind::new(n);

    for i in 0..n {
        map.remove(&i);
        for &to in &t[i] {
            if i < to {
                map.insert(to);
            } else {
                uf.merge(i, to);
            }
        }

        if uf.is_same(i, 0) && uf.size(0) == i + 1 {
            println!("{}", map.len());
        } else {
            println!("-1")
        }
    }
}
