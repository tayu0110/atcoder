use itertools::Itertools;
use proconio::*;

fn dfs(
    now: usize,
    par: usize,
    res: &mut Vec<usize>,
    reached: &mut [bool],
    t: &Vec<Vec<usize>>,
) -> bool {
    reached[now] = true;
    if now == t.len() - 1 {
        res.push(now + 1);
        return true;
    }

    for &to in &t[now] {
        if to != par && !reached[to] && dfs(to, now, res, reached, t) {
            res.push(now + 1);
            return true;
        }
    }

    false
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut res = vec![];
    let mut reached = vec![false; n];
    dfs(0, 0, &mut res, &mut reached, &t);

    println!("{}", res.iter().rev().join(" "))
}
