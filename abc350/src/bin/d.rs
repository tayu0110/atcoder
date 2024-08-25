use std::collections::HashSet;

use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut uf = UnionFind::new(n);
    for &(a, b) in &p {
        uf.merge(a - 1, b - 1);
    }

    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(uf.root(i));
    }

    let mut t = vec![0; n];
    for &root in &set {
        let sz = uf.size(root);
        t[root] = sz * (sz - 1) / 2;
    }

    for (a, _) in p {
        t[uf.root(a - 1)] -= 1;
    }

    println!("{}", t.into_iter().sum::<usize>())
}
