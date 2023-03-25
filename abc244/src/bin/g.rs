use itertools::Itertools;

fn dfs(now: usize, s: &mut [u32], checked: &mut [bool], res: &mut Vec<usize>, t: &Vec<Vec<usize>>) {
    checked[now] = true;
    res.push(now);
    s[now] ^= 1;

    for to in &t[now] {
        if !checked[*to] {
            dfs(*to, s, checked, res, t);
            res.push(now);
            s[now] ^= 1;
            if s[*to] == 1 {
                res.push(*to);
                res.push(now);
                s[*to] ^= 1;
                s[now] ^= 1;
            }
        }
    }
}

fn main() {
    proconio::input! {n: usize, m: usize, p: [(usize, usize); m], s: proconio::marker::Chars}

    let mut t = vec![vec![]; n+1];
    for (u, v) in p {
        t[u].push(v);
        t[v].push(u);
    }

    let mut s = vec![0].into_iter().chain(s.into_iter().map(|c| c.to_digit(10).unwrap())).collect::<Vec<_>>();
    let mut checked = vec![false; n+1];
    let mut res = vec![];

    dfs(1, &mut s, &mut checked, &mut res, &t);
    if s[1] == 1 {
        res.push(t[1][0]);
        res.push(1);
        res.push(t[1][0]);
    }

    println!("{}", res.len());
    println!("{}", res.iter().join(" "));
}