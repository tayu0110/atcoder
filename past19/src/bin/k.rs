use proconio::*;

fn dfs(now: usize, par: usize, a: &[u16], t: &[Vec<usize>], memo: &mut [Vec<Vec<u16>>]) {
    if memo[now][0].is_empty() {
        let mut m = vec![0];
        for &to in &t[now] {
            if to != par {
                dfs(to, now, a, t, memo);
                let len = m.len();
                m.resize(len + memo[to][0].len().max(memo[to][1].len()) - 1, 0);
                for i in (0..len).rev() {
                    for j in 0..memo[to][0].len().max(memo[to][1].len()) {
                        if j < memo[to][0].len() {
                            m[i + j] = m[i + j].max(m[i] + memo[to][0][j]);
                        }
                        if j < memo[to][1].len() {
                            m[i + j] = m[i + j].max(m[i] + memo[to][1][j]);
                        }
                    }
                }
            }
        }
        memo[now][0] = m;
    }
    if memo[now][1].is_empty() {
        let mut m = vec![0, a[now]];
        for &to in &t[now] {
            if to != par {
                dfs(to, now, a, t, memo);
                let len = m.len();
                m.resize(len + memo[to][0].len() - 1, 0);
                for i in (0..len).rev() {
                    for j in 0..memo[to][0].len() {
                        m[i + j] = m[i + j].max(m[i] + memo[to][0][j]);
                    }
                }
            }
        }
        memo[now][1] = m;
    }
}

fn main() {
    input! {n: usize, k: usize, e: [(usize, usize); n - 1], a: [u16; n]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut memo = vec![vec![vec![]; 2]; n];
    dfs(0, 0, &a, &t, &mut memo);
    if memo[0][0].len().max(memo[0][1].len()) <= k {
        println!("-1")
    } else {
        let mut res = 0;
        for i in 0..2 {
            if k < memo[0][i].len() {
                res = res.max(memo[0][i][k])
            }
        }
        println!("{res}");
    }
}
