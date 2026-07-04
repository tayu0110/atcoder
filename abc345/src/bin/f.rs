use itertools::Itertools;
use proconio::*;

fn dfs(now: usize, par: &mut [Option<(usize, usize)>], t: &[Vec<(usize, usize)>]) {
    for &(to, i) in &t[now] {
        if par[to].is_none() {
            par[to] = Some((now, i));
            dfs(to, par, t);
        }
    }
}

fn main() {
    input! {n: usize, m: usize, mut k: usize, e: [(usize, usize); m]}

    if k == 0 {
        println!("Yes");
        println!("0");
        println!();
        return;
    }

    if k % 2 != 0 {
        println!("No");
        return;
    }

    let mut t = vec![vec![]; n];
    for (i, (u, v)) in e.into_iter().enumerate() {
        t[u - 1].push((v - 1, i));
        t[v - 1].push((u - 1, i));
    }

    let mut par = vec![None; n];
    for i in 0..n {
        if par[i].is_none() {
            par[i] = Some((usize::MAX, usize::MAX));
            dfs(i, &mut par, &t);
        }
    }

    let mut ins = vec![0; n];
    for i in 0..n {
        if let Some((par, _)) = par[i] {
            if par != usize::MAX {
                ins[par] += 1;
            }
        }
    }
    let mut nt = vec![];
    for i in 0..n {
        if ins[i] == 0 {
            nt.push(i);
        }
    }

    let mut res = vec![];
    let mut is_black = vec![false; n];
    while let Some(now) = nt.pop() {
        if !is_black[now] {
            if let Some((par, i)) = par[now].filter(|p| p.0 != usize::MAX) {
                k -= 1;
                ins[par] -= 1;
                if ins[par] == 0 {
                    nt.push(par);
                }
                res.push(i + 1);
                if !is_black[par] {
                    k -= 1;
                } else {
                    k += 1;
                }
                is_black[par] = !is_black[par];
            }
        } else if let Some((par, _)) = par[now].filter(|p| p.0 != usize::MAX) {
            ins[par] -= 1;
            if ins[par] == 0 {
                nt.push(par);
            }
        }

        if k == 0 {
            break;
        }
    }

    if k == 0 {
        println!("Yes");
        println!("{}", res.len());
        println!("{}", res.iter().join(" "));
    } else {
        println!("No")
    }
}
