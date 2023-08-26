use proconio::*;

fn dfs(
    now: usize,
    res: &mut i64,
    a: &[i64],
    reached: &mut [bool],
    memo: &mut [i64],
    t: &Vec<Vec<usize>>,
) -> i64 {
    if reached[now] {
        return memo[now];
    }
    reached[now] = true;
    let mut max = std::i64::MIN;
    for &to in &t[now] {
        max = max.max(dfs(to, res, a, reached, memo, t));
    }

    if max != std::i64::MIN {
        *res = (max - a[now]).max(*res);
    }

    memo[now] = max.max(a[now]);
    memo[now]
}

fn main() {
    input! {n: usize, m: usize, a: [i64; n], p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (x, y) in p {
        t[x - 1].push(y - 1);
    }

    let mut res = std::i64::MIN;
    let mut reached = vec![false; n];
    let mut memo = vec![std::i64::MIN; n];
    for i in 0..n {
        if !reached[i] {
            dfs(i, &mut res, &a, &mut reached, &mut memo, &t);
        }
    }
    println!("{}", res)
}
