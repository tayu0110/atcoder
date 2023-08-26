use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize}

    let mut v = vec![];
    let mut t = vec![];
    for _ in 0..n {
        input! {tm: usize, a: [usize]}
        v.push(tm);
        t.push(a);
    }

    let mut res = v[n - 1];
    let mut used = vec![false; n];
    let mut nt = VecDeque::new();
    nt.push_back(n - 1);
    while let Some(now) = nt.pop_front() {
        for to in t[now].iter().map(|&to| to - 1) {
            if used[to] {
                continue;
            }
            used[to] = true;
            res += v[to];
            nt.push_back(to);
        }
    }

    println!("{}", res)
}
