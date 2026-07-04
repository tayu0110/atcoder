use proconio::*;

fn dfs(now: usize, p: &[f64], memo: &mut [f64], checked: &mut [bool], t: &[Vec<(usize, f64)>]) {
    if now == memo.len() - 1 {
        memo[now] = p[now];
        return;
    }
    if checked[now] {
        return;
    }
    checked[now] = true;
    for &(to, w) in &t[now] {
        dfs(to, p, memo, checked, t);
        if memo[to] != f64::MIN {
            memo[now] = memo[now].max(memo[to] * w + p[now]);
        }
    }
}

fn main() {
    input! {n: usize, m: usize, p: [f64; n], e: [(usize, usize, f64); m]}

    let mut t = vec![vec![]; n];
    for (u, v, w) in e {
        t[u - 1].push((v - 1, w));
    }
    let mut memo = vec![f64::MIN; n];
    let mut checked = vec![false; n];
    dfs(0, &p, &mut memo, &mut checked, &t);
    if memo[0] == f64::MIN {
        println!("-1")
    } else {
        println!("{}", memo[0])
    }
}
