use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn bfs(root: usize, t: &[Vec<usize>]) -> Vec<usize> {
    let mut res = vec![std::usize::MAX; t.len()];
    res[root] = 0;
    let mut nt = VecDeque::new();
    nt.push_back(root);
    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            if res[to] == std::usize::MAX {
                res[to] = res[now] + 1;
                nt.push_back(to);
            }
        }
    }

    res
}

fn rec(now: usize, par: usize, t: &[Vec<usize>], memo: &mut Vec<Vec<(usize, usize)>>) -> usize {
    let mut res = 0;
    for &to in &t[now] {
        if to != par {
            let r = rec(to, now, t, memo);
            memo[now].push((r, to));
            res = r.max(res);
        }
    }

    res + 1
}

fn solve(
    now: usize,
    par: usize,
    cnt: &mut usize,
    memo: &Vec<Vec<(usize, usize)>>,
    res: &mut [usize],
) {
    let mut r = memo[now].clone();
    r.sort();
    *cnt += 1;
    res[now] = *cnt;

    for (_, to) in r {
        if to != par {
            solve(to, now, cnt, memo, res);
            *cnt += 1;
        }
    }
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let roots = {
        let dist = bfs(0, &t);
        let max = *dist.iter().max().unwrap();
        let root = dist.iter().position(|&v| v == max).unwrap();
        let dist = bfs(root, &t);
        let max = *dist.iter().max().unwrap();
        vec![root, dist.iter().position(|&v| v == max).unwrap()]
    };

    let mut r = vec![];
    for root in roots {
        let mut memo = vec![vec![]; n];
        rec(root, root, &t, &mut memo);

        let mut res = vec![0; n];
        let mut cnt = 0;
        solve(root, root, &mut cnt, &memo, &mut res);
        r.push(res);
    }

    let res = if r[0].iter().max().unwrap() < r[1].iter().max().unwrap() {
        r[0].clone()
    } else {
        r[1].clone()
    };

    println!("{}", res.iter().join(" "))
}
