use itertools::Itertools;
use proconio::*;

fn dfs(now: usize, par: usize, res: &mut [u32], t: &Vec<Vec<u32>>) -> u32 {
    let mut rank = 0;
    for &to in &t[now] {
        if to as usize == par {
            continue;
        }

        rank = rank.max(dfs(to as usize, now, res, t) + 1);
    }

    res[now] = rank;
    rank
}

#[fastout]
fn main() {
    input! {n: usize, r: usize}

    let mut t = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {a: u32, b: u32}

        t[a as usize - 1].push(b - 1);
        t[b as usize - 1].push(a - 1);
    }

    let mut res = vec![0; n];
    dfs(r - 1, r - 1, &mut res, &t);

    println!("{}", res.iter().join(" "))
}
