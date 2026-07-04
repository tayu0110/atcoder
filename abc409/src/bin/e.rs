use std::collections::VecDeque;

use proconio::*;

fn dfs(now: usize, par: usize, x: &[i64], t: &[Vec<(usize, i64)>], memo: &mut [Vec<i64>]) -> i64 {
    for (i, &(to, _)) in t[now].iter().enumerate() {
        if to != par {
            memo[now][i] = dfs(to, now, x, t, memo);
        }
    }

    memo[now].iter().filter(|&&m| m < i64::MAX).sum::<i64>() + x[now]
}

fn main() {
    input! {n: usize, mut x: [i64; n], e: [(usize, usize, i64); n - 1]}

    let mut t = vec![vec![]; n];
    for (u, v, w) in e {
        t[u - 1].push((v - 1, w));
        t[v - 1].push((u - 1, w));
    }

    let mut memo = vec![vec![]; n];
    for i in 0..n {
        memo[i].resize(t[i].len(), i64::MAX);
    }

    dfs(0, 0, &x, &t, &mut memo);

    let mut nt = VecDeque::new();
    nt.push_back(0);
    let mut res = 0;
    while let Some(now) = nt.pop_front() {
        for i in 0..t[now].len() {
            if memo[now][i] == i64::MAX {
                continue;
            }

            nt.push_back(t[now][i].0);
            res += memo[now][i].abs() * t[now][i].1;
            x[now] += memo[now][i];
            x[t[now][i].0] -= memo[now][i] as i64;
        }
    }

    assert!(x.iter().all(|&x| x == 0), "x: {x:?}");

    println!("{res}")
}
