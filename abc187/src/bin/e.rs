use itertools::Itertools;
use proconio::*;

fn gen_par(now: usize, par: &mut [usize], t: &Vec<Vec<usize>>) {
    for &to in &t[now] {
        if to == par[now] {
            continue;
        }
        par[to] = now;
        gen_par(to, par, t);
    }
}

fn dfs(now: usize, par: usize, sum: i64, res: &mut [i64], t: &Vec<Vec<usize>>) {
    res[now] += sum;
    for &to in &t[now] {
        if to == par {
            continue;
        }
        dfs(to, now, res[now], res, t);
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n-1], q: usize, p: [(usize, usize, i64); q]}

    let mut t = vec![vec![]; n];
    for &(a, b) in &e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut par = vec![std::usize::MAX; n];
    gen_par(0, &mut par, &t);

    let mut res = vec![0i64; n];
    for (t, ne, x) in p {
        let (mut a, mut b) = e[ne - 1];
        if t == 2 {
            std::mem::swap(&mut a, &mut b);
        }
        if par[a - 1] == b - 1 {
            res[a - 1] += x;
        } else {
            res[0] += x;
            res[b - 1] -= x;
        }
    }
    dfs(0, 0, 0, &mut res, &t);

    println!("{}", res.iter().join("\n"))
}
