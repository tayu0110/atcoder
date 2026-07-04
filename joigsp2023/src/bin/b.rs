use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize}

    let mut tmp = vec![vec![]; n + 1];
    let mut cnt = vec![vec![]; n + 1];
    let mut t = vec![vec![]; n + 1];
    for i in 1..n {
        input! {a: usize, p: [(usize, usize); a]}
        cnt[i].resize(a, 0);
        tmp[i].resize(a, 0);
        for (j, (p, q)) in p.into_iter().enumerate() {
            t[p].push((i, j));
            t[q].push((i, j));
        }
    }

    let mut res = vec![-1; n + 1];
    res[n] = 0;
    let mut nt = VecDeque::new();
    nt.push_back(n);
    while let Some(now) = nt.pop_front() {
        for &(to, e) in &t[now] {
            cnt[to][e] += 1;
            tmp[to][e] = tmp[to][e].max(res[now] + 1);
            if cnt[to][e] == 2 && res[to] < 0 {
                nt.push_back(to);
                res[to] = tmp[to][e];
            }
        }
    }

    println!("{}", res[1]);
}
