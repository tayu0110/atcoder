use std::mem::take;

use proconio::*;

fn dfs(
    now: usize,
    r: usize,
    c: usize,
    dig: usize,
    m: usize,
    t: &[Vec<usize>],
    memo: &mut Vec<Vec<Vec<usize>>>,
) {
    let now = (now * 10 % m + t[r][c]) % m;
    if r + c + 1 == dig {
        memo[r][c].push(now);
        return;
    }

    if r + 1 < t.len() {
        dfs(now, r + 1, c, dig, m, t, memo);
    }
    if c + 1 < t[0].len() {
        dfs(now, r, c + 1, dig, m, t, memo);
    }
}

fn dfs2(
    now: usize,
    r: usize,
    c: usize,
    cur: usize,
    dig: usize,
    m: usize,
    mut ten: usize,
    t: &[Vec<usize>],
    memo: &mut Vec<Vec<Vec<usize>>>,
) {
    let now = (now + (ten * t[r][c])) % m;
    ten *= 10;
    ten %= m;

    if cur + 1 == dig {
        if r > 0 {
            memo[r - 1][c].push(now);
        }
        if c > 0 {
            memo[r][c - 1].push(now);
        }
        return;
    }

    if r > 0 {
        dfs2(now, r - 1, c, cur + 1, dig, m, ten, t, memo);
    }
    if c > 0 {
        dfs2(now, r, c - 1, cur + 1, dig, m, ten, t, memo);
    }
}

fn main() {
    input! {n: usize, m: usize, a: [[usize; n]; n]}

    if n == 1 {
        println!("{}", a[0][0] % m);
        return;
    }

    let mut m1 = vec![vec![vec![]; n]; n];
    dfs(0, 0, 0, n, m, &a, &mut m1);

    let mut m2 = vec![vec![vec![]; n]; n];
    dfs2(0, n - 1, n - 1, 0, n - 1, m, 1, &a, &mut m2);

    let mut res = 0;
    let mut ten = 1;
    for _ in 0..n - 1 {
        ten *= 10;
        ten %= m;
    }
    for i in 0..n {
        for j in 0..n {
            if m1[i][j].is_empty() || m2[i][j].is_empty() {
                continue;
            }

            let mut m1 = take(&mut m1[i][j]);
            let mut m2 = take(&mut m2[i][j]);
            m1.sort_unstable();
            m1.dedup();
            m2.sort_unstable();
            m2.dedup();

            m1.iter_mut().for_each(|m1| *m1 = *m1 * ten % m);

            for m1 in m1 {
                let pos = m2.partition_point(|m2| m1 + m2 < m);
                if pos > 0 {
                    res = res.max((m1 + m2[pos - 1]) % m);
                }
                res = res.max((m1 + m2[m2.len() - 1]) % m);
            }
        }
    }

    println!("{res}");
}
