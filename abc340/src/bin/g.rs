use proconio::*;
use rustc_hash::FxHashMap;

fn compress(
    now: usize,
    par: usize,
    a: &mut Vec<usize>,
    t: &[Vec<usize>],
    leaf: &mut Vec<usize>,
    ct: &mut Vec<FxHashMap<usize, Vec<usize>>>,
) {
    if leaf[a[now]] == usize::MAX {
        leaf[a[now]] = now;
    }

    for &to in &t[now] {
        if to == par {
            continue;
        }

        if a[now] == a[to] {
            ct[a[now]].entry(now).or_insert(vec![]).push(to);
            ct[a[now]].entry(to).or_insert(vec![]).push(now);
            leaf[a[now]] = to;
            compress(to, now, a, t, leaf, ct);
            leaf[a[now]] = now;
        } else {
            let dum = a.len();
            a.push(a[now]);
            ct.push(FxHashMap::default());
            ct[a[to]].entry(leaf[a[to]]).or_insert(vec![]).push(to);
            let ol = leaf[a[to]];
            leaf[a[now]] = dum;
            leaf[a[to]] = to;
            compress(to, now, a, t, leaf, ct);
            leaf[a[to]] = ol;
            leaf[a[now]] = now;
        }
    }
}

fn dfs(now: usize, par: usize, t: &mut FxHashMap<usize, Vec<usize>>) -> usize {
    let mut res = 1;
    let children = t.get(&now).unwrap();
    for &to in t.get(&now).unwrap() {
        if to == par {
            continue;
        }
    }
    todo!()
}

fn main() {
    input! {n: usize, mut a: [usize; n], e: [(usize, usize); n - 1]}
    a.iter_mut().for_each(|a| *a -= 1);

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut leaf = vec![usize::MAX; n];
    let mut tree = vec![FxHashMap::default(); n];
    compress(0, 0, &mut a, &t, &mut leaf, &mut tree);

    for t in tree {}
}
