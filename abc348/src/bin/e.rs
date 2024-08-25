use proconio::*;

fn dfs(
    now: usize,
    par: usize,
    c: &[usize],
    t: &[Vec<usize>],
    memo: &mut [Vec<(usize, usize)>],
) -> (usize, usize) {
    let mut res = 0;
    let mut sum = 0;

    for &to in &t[now] {
        if to == par {
            memo[now].push((usize::MAX, usize::MAX));
        } else {
            let (s, r) = dfs(to, now, c, t, memo);
            memo[now].push((r, s));
            res += r + s;
            sum += s;
        }
    }

    (sum + c[now], res + c[now])
}

fn dfs2(
    now: usize,
    pr: usize,
    ps: usize,
    c: &[usize],
    t: &[Vec<usize>],
    memo: &mut [Vec<(usize, usize)>],
) {
    let mut prop = c[now];
    let mut sum = c[now];
    let mut par = usize::MAX;
    for i in 0..t[now].len() {
        if memo[now][i] == (usize::MAX, usize::MAX) {
            memo[now][i] = (pr, ps);
            par = i;
        }
        let (r, s) = memo[now][i];
        prop += r + s;
        sum += s;
    }

    for i in 0..t[now].len() {
        if par == i {
            continue;
        }

        let (r, s) = memo[now][i];
        dfs2(t[now][i], prop - r - s, sum - s, c, t, memo);
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1], c: [usize; n]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut memo = vec![vec![]; n];
    dfs(0, 0, &c, &t, &mut memo);
    dfs2(0, 0, 0, &c, &t, &mut memo);

    println!(
        "{}",
        memo.into_iter()
            .map(|v| v.into_iter().map(|v| v.0).sum::<usize>())
            .min()
            .unwrap()
    );
}
