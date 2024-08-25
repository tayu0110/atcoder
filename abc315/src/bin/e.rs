use itertools::Itertools;
use proconio::*;
use std::collections::VecDeque;

fn dfs(now: usize, par: usize, need: &mut [bool], t: &Vec<Vec<usize>>) {
    need[now] = true;
    for &to in &t[now] {
        if to != par && !need[to] {
            dfs(to, par, need, t);
        }
    }
}

fn main() {
    input! {n: usize}

    let mut p = vec![];
    for _ in 0..n {
        input! {c: usize, np: [usize; c]}
        p.push(np);
    }

    let mut t = vec![vec![]; n];
    let mut rt = vec![vec![]; n];
    let mut ins = vec![0; n];
    for i in 0..n {
        for &nei in &p[i] {
            t[nei - 1].push(i);
            rt[i].push(nei - 1);
            ins[i] += 1;
        }
    }

    let mut need = vec![false; n];
    dfs(0, 0, &mut need, &rt);

    let mut nt = VecDeque::new();
    for i in 0..n {
        if ins[i] == 0 && need[i] {
            nt.push_back(i);
        }
    }

    let mut res = vec![];
    let mut used = vec![false; n];
    while let Some(now) = nt.pop_front() {
        if used[now] {
            continue;
        }
        used[now] = true;
        res.push(now);
        if now == 0 {
            break;
        }
        for &to in &t[now] {
            ins[to] -= 1;
            if ins[to] == 0 && need[to] && !used[to] {
                nt.push_back(to);
            }
        }
    }
    res.pop();

    println!("{}", res.iter().map(|v| v + 1).join(" "))
}
