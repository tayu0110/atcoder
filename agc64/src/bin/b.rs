use itertools::Itertools;
use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, char); m], s: marker::Chars}

    let mut uf = UnionFind::new(n);
    let mut t = vec![vec![]; n];
    let mut res = vec![];
    for (i, (a, b, c)) in e.into_iter().enumerate() {
        t[a - 1].push((b - 1, c, i + 1));
        t[b - 1].push((a - 1, c, i + 1));

        if s[a - 1] == c && s[a - 1] == s[b - 1] && !uf.is_same(a - 1, b - 1) {
            uf.merge(a - 1, b - 1);
            res.push(i + 1);
        }
    }

    let mut nt = vec![];
    let mut checked = vec![false; n];
    for now in 0..n {
        if uf.size(now) > 1 {
            nt.push(now);
        }
    }

    while let Some(now) = nt.pop() {
        if checked[now] {
            continue;
        }
        checked[now] = true;

        for &(to, col, i) in &t[now] {
            if checked[to] {
                continue;
            }

            if uf.size(to) > 1 {
                continue;
            }

            if s[to] != col {
                continue;
            }

            if uf.is_same(to, now) {
                continue;
            }

            uf.merge(now, to);
            res.push(i);
            nt.push(to);
        }
    }

    if (0..n).any(|i| uf.size(i) == 1) {
        println!("No");
        return;
    }

    for now in 0..n {
        for &(to, _, i) in &t[now] {
            if uf.is_same(now, to) {
                continue;
            }

            uf.merge(now, to);
            res.push(i);
        }
    }

    res.sort();
    res.dedup();
    assert_eq!(res.len(), n - 1);

    println!("Yes");
    println!("{}", res.iter().join(" "));
}
