use itertools::Itertools;
use proconio::*;

fn dfs(now: usize, used: &mut [bool], res: &mut Vec<usize>, t: &Vec<Vec<usize>>) {
    for &to in &t[now] {
        if !used[to] {
            dfs(to, used, res, t);
        }
    }
    used[now] = true;
    res.push(now + 1);
}

fn main() {
    input! {n: usize}

    let t = (0..n)
        .map(|_| {
            input! {c: usize, p: [usize; c]}
            p.into_iter().map(|p| p - 1).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = vec![];
    let mut used = vec![false; n];
    dfs(0, &mut used, &mut res, &t);
    res.pop();
    println!("{}", res.iter().join(" "))
}
