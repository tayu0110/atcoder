use std::cmp::Reverse;

use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashMap;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, q: usize}

    let mut uf = UnionFind::new(n);
    let mut map = FxHashMap::default();
    for i in 0..n {
        map.insert(i, vec![i as i32]);
    }
    let mut res = vec![];
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {u: usize, v: usize}
            if uf.is_same(u - 1, v - 1) {
                continue;
            }

            let mut uv = map.remove(&uf.root(u - 1)).unwrap();
            let vv = map.remove(&uf.root(v - 1)).unwrap();
            uv.extend(vv);
            uv.sort_unstable_by_key(|v| Reverse(*v));
            uv.resize(10, -2);

            uf.merge(u - 1, v - 1);
            map.insert(uf.root(u - 1), uv);
        } else {
            input! {v: usize, k: usize}
            let slice = map.get(&uf.root(v - 1)).unwrap();
            res.push(*slice.get(k - 1).unwrap_or(&-2) + 1)
        }
    }

    println!("{}", res.iter().join("\n"))
}
