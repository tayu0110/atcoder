use std::collections::HashMap;

use graph::path_query;
use itertools::Itertools;
use proconio::input;
use segtree::SegmentTree;

// false: lower, true: upper
fn binary_search(tar: usize, bound: bool, v: &[(usize, i64)]) -> i32 {
    let (mut l, mut r) = (-1, v.len() as i32);
    while r - l > 1 {
        let m = (r + l) / 2;
        if bound {
            if tar < v[m as usize].0 {
                r = m;
            } else {
                l = m;
            }
        } else {
            if tar <= v[m as usize].0 {
                r = m;
            } else {
                l = m;
            }
        }
    }
    l
}

fn main() {
    input! {n: usize, q: usize, p: [(usize, usize, usize, i64); n-1]}

    let mut t = vec![vec![]; n];
    for &(a, b, _, d) in &p {
        t[a - 1].push((b - 1, d));
        t[b - 1].push((a - 1, d));
    }

    let tree = t.try_into().unwrap();
    let (query, index) = path_query(&tree);

    let mut st = SegmentTree::new(n, 0, |l, r| l + r);
    let mut set = HashMap::new();
    let mut color = vec![vec![]; n];
    for &(a, b, c, d) in &p {
        let (na, nb) = (index(a - 1), index(b - 1));

        if na.abs_diff(nb) == 1 {
            st.set(na.min(nb), d);
            color[c].push((na.min(nb), d));
        }
        set.insert((na.min(nb), na.max(nb)), (c, d));
    }
    eprintln!("set: {set:?}");

    color.iter_mut().for_each(|v| v.sort());
    for v in color.iter_mut() {
        if !v.is_empty() {
            for i in 0..v.len() - 1 {
                v[i + 1].1 += v[i].1;
            }
        }
    }

    let mut ans = vec![];
    for _ in 0..q {
        input! {x: usize, y: i64, u: usize, v: usize}
        let (u, v) = (u - 1, v - 1);
        eprintln!("u: {u}, v: {v}");

        let mut res = 0;
        let path = query(u, v);
        let (mut pu, mut pv) = (usize::MAX, usize::MAX);
        for (u, v) in path
            .into_iter()
            .map(|(u, v)| (index(u), index(v)))
            .map(|(u, v)| (u.min(v), u.max(v)))
        {
            eprintln!("in: u: {u}, v: {v}");
            if pu < usize::MAX {
                for u in vec![u, v] {
                    for v in vec![pu, pv] {
                        eprintln!("inner: u: {u}, v: {v}");
                        if let Some(&(c, d)) = set.get(&(u.min(v), u.max(v))) {
                            if c == x {
                                res += d;
                            } else {
                                res += y;
                            }
                        }
                    }
                }
            }
            (pu, pv) = (u, v);
            if pu == pv {
                pv = usize::MAX;
            }

            let (low, up) = (
                binary_search(u, false, &color[x]),
                binary_search(v, true, &color[x]),
            );
            eprintln!("low: {low}, up: {up}");
            if up - low > 0 {
                let mut t = color[x][up as usize].1;
                if low >= 0 {
                    t -= color[x][low as usize].1;
                }
                res -= t;
                res += y * (up - low) as i64;
            }
            res += st.foldl(u.min(v), u.max(v));
            eprintln!("res: {res}");
        }

        // println!("{res}")
        ans.push(res);
    }

    println!("{}", ans.iter().join("\n"))
}
