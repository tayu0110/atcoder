#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn dfs(now: usize, par: usize, x: &mut usize, res: &mut [(usize, usize)], t: &Vec<Vec<usize>>) -> (usize, usize) {
    let (mut l, mut r) = (std::usize::MAX, 0);

    for &to in &t[now] {
        if to == par {
            continue;
        }

        let (s, t) = dfs(to, now, x, res, t);
        l = std::cmp::min(l, s);
        r = std::cmp::max(r, t);
    }

    if l == std::usize::MAX {
        l = *x;
        r = *x;
        *x += 1;
    }

    res[now] = (l, r);
    (l, r)
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let mut x = 1;
    let mut res = vec![(0, 0); n];
    dfs(0, 0, &mut x, &mut res, &t);

    for (l, r) in res {
        println!("{} {}", l, r);
    }
}
