use std::collections::HashMap;

use fenwick_tree::FenwickTree;
use graph::path_query;
use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize, i64); n-1], q: usize, query: [(usize, usize, usize); q]}

    let mut t = vec![vec![]; n];
    for &(u, v, _) in &p {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let tree = t.into();
    let (path_query, index) = path_query(&tree);

    let mut ft = FenwickTree::new(n, 0);
    let mut map = HashMap::new();
    for &(u, v, w) in &p {
        let (l, r) = (index(u - 1), index(v - 1));
        let (l, r) = (l.min(r), l.max(r));

        if r - l > 1 {
            map.insert((l, r), w);
        } else {
            ft.add(l, w);
        }
    }

    for (t, i, w) in query {
        if t == 1 {
            let (u, v, _) = p[i - 1];
            let (l, r) = (index(u - 1), index(v - 1));
            let (l, r) = (l.min(r), l.max(r));

            if r - l > 1 {
                map.insert((l, r), w as i64);
            } else {
                let now = ft.get_sum(l, r);
                ft.add(l, w as i64 - now);
            }
        } else {
            let path = path_query(i - 1, w - 1);
            let mut res = 0;
            let mut prev = std::usize::MAX;
            for (u, v) in path {
                res += ft.get_sum(u.min(v), u.max(v));
                if prev != std::usize::MAX {
                    res += map.get(&(prev.min(u), prev.max(u))).unwrap();
                }
                prev = v;
            }

            println!("{}", res);
        }
    }
}
