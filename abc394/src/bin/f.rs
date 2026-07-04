use std::cmp::Reverse;

use proconio::*;

fn dfs(now: usize, par: usize, t: &[Vec<usize>], memo: &mut [Vec<usize>]) -> usize {
    for i in 0..t[now].len() {
        let to = t[now][i];
        if to != par {
            memo[now][i] = dfs(to, now, t, memo);
        }
    }

    let mut m = memo[now]
        .iter()
        .copied()
        .filter(|&m| m > 0)
        .collect::<Vec<_>>();
    if m.len() >= 3 {
        m.sort_unstable_by_key(|&m| Reverse(m));
        m[0] + m[1] + m[2] + 1
    } else {
        1
    }
}

fn dfs2(now: usize, prop: usize, t: &[Vec<usize>], memo: &mut [Vec<usize>], res: &mut [usize]) {
    let mut par = usize::MAX;
    for i in 0..t[now].len() {
        if memo[now][i] == 0 {
            par = i;
            memo[now][i] = prop;
        }
    }

    let mut m = memo[now]
        .iter()
        .copied()
        .enumerate()
        .map(|(i, m)| (m, i))
        .collect::<Vec<_>>();
    m.sort_unstable_by_key(|m| Reverse(m.0));
    if m.len() >= 4 {
        res[now] = m[..4].iter().copied().map(|m| m.0).sum::<usize>() + 1;
    }

    for i in 0..t[now].len() {
        if i != par {
            let to = t[now][i];
            if m.len() >= 4 {
                if m[..3].iter().any(|m| m.1 == i) {
                    let s = m[..4]
                        .iter()
                        .filter(|m| m.1 != i)
                        .map(|m| m.0)
                        .sum::<usize>();
                    dfs2(to, s + 1, t, memo, res);
                } else {
                    let s = m[..3].iter().map(|m| m.0).sum::<usize>();
                    dfs2(to, s + 1, t, memo, res);
                }
            } else {
                dfs2(to, 1, t, memo, res);
            }
        }
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    let mut memo = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
        memo[a - 1].push(0);
        memo[b - 1].push(0);
    }

    dfs(0, 0, &t, &mut memo);

    let mut res = vec![0; n];
    dfs2(0, 0, &t, &mut memo, &mut res);

    let &max = res.iter().max().unwrap();
    if max > 0 {
        println!("{max}")
    } else {
        println!("-1")
    }
}
