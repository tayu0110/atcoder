#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

const MAX: usize = 1000_000;

fn dfs(now: usize, res: &mut usize, reached: &mut [bool], t: &Vec<Vec<usize>>) {
    if reached[now] {
        return;
    }
    reached[now] = true;
    *res += 1;

    if *res > MAX {
        println!("{}", MAX);
        std::process::exit(0);
    }

    for &to in &t[now] {
        if !reached[to] {
            dfs(to, res, reached, t);
        }
    }

    reached[now] = false;
}

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut res = 0;
    let mut reached = vec![false; n];
    dfs(0, &mut res, &mut reached, &t);

    println!("{}", res);
}
