use itertools::Itertools;
use proconio::*;
use static_modint::{combination, Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn dfs(
    now: usize,
    par: usize,
    size: &mut Vec<Vec<usize>>,
    memo: &mut Vec<Vec<Modint>>,
    com: &impl Fn(u32, u32) -> Modint,
    t: &Vec<Vec<usize>>,
) -> (usize, Modint) {
    let mut res = 0;
    let mut c = Modint::one();

    for &to in &t[now] {
        if to == par {
            size[now].push(std::usize::MAX);
            memo[now].push(Modint::zero());
            continue;
        }

        let (l, r) = dfs(to, now, size, memo, com, t);
        res += l;
        size[now].push(l);

        c *= com(res as u32, l as u32) * r;
        memo[now].push(r);
    }

    (res + 1, c)
}

fn dfs2(
    now: usize,
    prps: usize,
    prpc: Modint,
    res: &mut Vec<Modint>,
    size: &mut Vec<Vec<usize>>,
    memo: &mut Vec<Vec<Modint>>,
    com: &impl Fn(u32, u32) -> Modint,
    t: &Vec<Vec<usize>>,
) {
    let mut total_size = 0;
    let mut c = Modint::one();

    let mut par = std::usize::MAX;
    for i in 0..t[now].len() {
        if size[now][i] == std::usize::MAX {
            size[now][i] = prps;
            memo[now][i] = prpc;
            par = i;
        }

        total_size += size[now][i];
        c *= com(total_size as u32, size[now][i] as u32) * memo[now][i];
    }

    res[now] = c;

    for i in 0..t[now].len() {
        if par != i {
            let d = com(total_size as u32, size[now][i] as u32) * memo[now][i];
            dfs2(
                t[now][i],
                total_size + 1 - size[now][i],
                c / d,
                res,
                size,
                memo,
                com,
                t,
            );
        }
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut size = vec![vec![]; n];
    let mut memo = vec![vec![]; n];
    let com = combination(n as u32 + 10);
    dfs(0, 0, &mut size, &mut memo, &com, &t);

    let mut res = vec![Modint::zero(); n];
    dfs2(
        0,
        0,
        Modint::zero(),
        &mut res,
        &mut size,
        &mut memo,
        &com,
        &t,
    );

    println!("{}", res.iter().join("\n"))
}
