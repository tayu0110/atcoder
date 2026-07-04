use proconio::*;

fn dfs(now: usize, par: usize, t: &[Vec<usize>], memo: &mut [Vec<[i32; 2]>]) -> [i32; 2] {
    if t[now].len() == 1 && t[now][0] == par {
        return [0; 2];
    }

    let mut max = 0;
    for (i, &to) in t[now].iter().enumerate() {
        if par == to {
            continue;
        }

        memo[now][i] = dfs(to, now, t, memo);
        max += memo[now][i][0].max(memo[now][i][1]);
    }

    let mut res = [max, i32::MIN];
    for (i, &to) in t[now].iter().enumerate() {
        if par != to {
            res[1] = res[1].max(max - memo[now][i][0].max(memo[now][i][1]) + memo[now][i][0] + 1);
        }
    }
    res
}

fn dfs2(n: i32, now: usize, prop: [i32; 2], t: &[Vec<usize>], memo: &mut [Vec<[i32; 2]>]) -> i32 {
    let mut par = usize::MAX;
    let mut max = 0;
    for i in 0..t[now].len() {
        if memo[now][i] == [i32::MIN; 2] {
            memo[now][i] = prop;
            par = i;
        }
        max += memo[now][i][0].max(memo[now][i][1]);
    }

    let mut dp = vec![[0; 2]; t[now].len() + 1];
    for i in 0..t[now].len() {
        dp[i + 1][0] = dp[i][0] + memo[now][i][0].max(memo[now][i][1]);
        dp[i + 1][1] =
            (dp[i][0] + memo[now][i][0] + 1).max(dp[i][1] + memo[now][i][0].max(memo[now][i][1]));
    }
    let mut dp2 = vec![[0; 2]; t[now].len() + 1];
    for i in (0..t[now].len()).rev() {
        dp2[i][0] = dp2[i + 1][0] + memo[now][i][0].max(memo[now][i][1]);
        dp2[i][1] = (dp2[i + 1][0] + memo[now][i][0] + 1)
            .max(dp2[i + 1][1] + memo[now][i][0].max(memo[now][i][1]));
    }

    let mut res = 0;
    for (i, &to) in t[now].iter().enumerate() {
        if i == par {
            continue;
        }

        let m1 = max - memo[now][i][0].max(memo[now][i][1]);
        let m2 = (dp[i][0] + dp2[i + 1][1]).max(dp[i][1] + dp2[i + 1][0]);
        // eprintln!("now: {now}, to: {to}, m1: {m1}, m2: {m2}");
        res += dfs2(n, to, [m1, m2], t, memo);
    }

    // eprintln!("now: {now}, memo: {:?}, max: {max}", memo[now]);
    if n == max {
        res + 1
    } else {
        res
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut memo = vec![vec![]; n];
    for i in 0..n {
        memo[i].resize(t[i].len(), [i32::MIN; 2]);
    }

    let [m1, m2] = dfs(0, 0, &t, &mut memo);
    let max = m1.max(m2);
    // eprintln!("max: {max}");
    // eprintln!("memo: {memo:?}");
    println!("{}", dfs2(max, 0, [0; 2], &t, &mut memo))
}
