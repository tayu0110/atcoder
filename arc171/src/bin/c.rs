use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn dfs(
    now: usize,
    par: usize,
    t: &Vec<Vec<usize>>,
    fsum: &[Modint],
    memo: &mut Vec<Vec<Option<Modint>>>,
) -> Modint {
    let mut res = Modint::one();
    for (i, &to) in t[now].iter().enumerate() {
        if to == par {
            continue;
        }

        let r = dfs(to, now, t, fsum, memo);
        memo[now][i] = Some(r);
        res *= r;
    }

    res * fsum[t[now].len() - 1]
}

fn dfs2(
    now: usize,
    par: usize,
    p: Modint,
    t: &Vec<Vec<usize>>,
    fsum: &[Modint],
    memo: &mut Vec<Vec<Option<Modint>>>,
) {
    let mut prop = Modint::one();
    for i in 0..t[now].len() {
        if memo[now][i].is_none() {
            memo[now][i] = Some(p);
        }
        prop *= memo[now][i].unwrap();
    }
    prop *= fsum[t[now].len() - 1];
    for (i, &to) in t[now].iter().enumerate() {
        if to != par {
            dfs2(to, now, prop / memo[now][i].unwrap(), t, fsum, memo);
        }
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut memo = vec![vec![]; n];
    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
        memo[u - 1].push(None);
        memo[v - 1].push(None);
    }

    let mut frac = vec![Modint::one(); n + 1];
    for i in 1..n + 1 {
        frac[i] = frac[i - 1] * Modint::raw(i as u32);
    }

    let mut perm = vec![Modint::zero(); n + 1];
    for i in 0..n + 1 {
        perm[i] = frac[n] / frac[n - i];
    }
    let mut fsum = perm.clone();
    for i in 0..n {
        fsum[i + 1] = fsum[i] + fsum[i + 1];
    }

    dfs(0, 0, &t, &fsum, &mut memo);
    dfs2(0, 0, Modint::one(), &t, &fsum, &mut memo);

    let mut res = Modint::zero();
    for i in 0..n {
        let mut r = fsum[t[i].len()];
        for j in 0..t[i].len() {
            r *= memo[i][j].unwrap();
        }
        res += r;
    }

    println!("{res}");
}
