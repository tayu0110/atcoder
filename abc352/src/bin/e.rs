use std::collections::HashSet;

use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize}

    let mut t = vec![];
    for _ in 0..m {
        input! {k: usize, c: usize, mut a: [usize; k]}
        a.iter_mut().for_each(|a| *a -= 1);
        t.push((c, a));
    }

    t.sort_unstable_by_key(|t| t.0);

    let mut res = 0;
    let mut uf = UnionFind::new(n);
    for (c, t) in t {
        for v in t.windows(2) {
            let (u, v) = (v[0], v[1]);
            if !uf.is_same(u, v) {
                uf.merge(u, v);
                res += c;
            }
        }
    }

    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(uf.root(i));
    }

    if set.len() > 1 {
        println!("-1");
        return;
    }

    println!("{res}")
}
