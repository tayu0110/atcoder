use proconio::*;

const W: usize = 5000;

fn dfs(
    now: usize,
    w: &[usize],
    t: &[Vec<usize>],
    can_reachable: &mut [usize],
    a: &[usize],
    res: &mut [usize],
) -> usize {
    if can_reachable[now] != usize::MAX {
        return can_reachable[now];
    }

    if t[now].is_empty() {
        res[now] = a[now];
        can_reachable[now] = 0;
        return 0;
    }

    let mut dp = vec![usize::MAX; w[now]];
    dp[0] = 0;
    for &to in &t[now] {
        let c = dfs(to, w, t, can_reachable, a, res) + 1;
        for i in (0..w[now]).rev() {
            if dp[i] == usize::MAX {
                continue;
            }
            if i + w[to] >= w[now] {
                continue;
            }

            if dp[i + w[to]] == usize::MAX {
                dp[i + w[to]] = dp[i] + c;
            } else if dp[i + w[to]] < dp[i] + c {
                dp[i + w[to]] = dp[i] + c;
            } else if dp[i + w[to]] == dp[i] + c {
            }
        }
    }

    let &max = dp.iter().filter(|d| **d != usize::MAX).max().unwrap();
    res[now] = a[now] * (max + 1);
    can_reachable[now] = max;
    can_reachable[now]
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m], w: [usize; n], mut a: [usize; n]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        if w[u - 1] > w[v - 1] {
            t[u - 1].push(v - 1);
        } else if w[u - 1] < w[v - 1] {
            t[v - 1].push(u - 1);
        }
    }
    let mut category = vec![vec![]; W + 1];
    for i in 0..n {
        category[w[i]].push(i);
    }

    let mut can_reachable = vec![usize::MAX; n];
    let mut res = vec![0; n];
    for i in 1..=W {
        for &now in &category[i] {
            if can_reachable[now] == usize::MAX {
                dfs(now, &w, &t, &mut can_reachable, &a, &mut res);
            }
        }
    }

    println!("{}", res.iter().sum::<usize>())
}
