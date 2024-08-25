use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashSet;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, mut e: [(usize, usize); m], q: usize, mut query: [(usize, usize, usize); q]}
    e.iter_mut().for_each(|v| {
        v.0 -= 1;
        v.1 -= 1;
    });
    query.iter_mut().for_each(|v| {
        v.1 -= 1;
        v.2 -= 1;
    });

    let mut uf = UnionFind::new(n);
    let mut set = FxHashSet::default();
    for &(a, b) in &e {
        set.insert((a, b));
    }
    for &(ty, x, y) in &query {
        if ty == 1 {
            set.insert((x, y));
        } else if ty == 2 {
            set.remove(&(x, y));
        }
    }

    for &(x, y) in &set {
        uf.merge(x, y);
    }

    let mut res = vec![];
    for &(ty, x, y) in query.iter().rev() {
        if ty == 1 {
            set.remove(&(x, y));
            uf.clear();
            for &(x, y) in &set {
                uf.merge(x, y);
            }
        } else if ty == 2 {
            set.insert((x, y));
            uf.merge(x, y);
        } else {
            if uf.is_same(x, y) {
                res.push("Yes");
            } else {
                res.push("No");
            }
        }
    }

    println!("{}", res.iter().rev().join("\n"))
}
