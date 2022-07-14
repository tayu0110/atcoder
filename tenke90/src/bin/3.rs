use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]};

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let mut nt = VecDeque::new();
    nt.push_back((0, 0));
    let mut d = vec![-1; n];
    while let Some((now, nd)) = nt.pop_front() {
        if d[now] >= 0 {
            continue;
        }
        d[now] = nd;
        for to in &t[now] {
            if d[*to] >= 0 {
                continue;
            }
            nt.push_back((*to, nd + 1));
        }
    }

    let start = {
        let mut idx = 0;
        let max = *d.iter().max().unwrap();
        for (i, v) in d.into_iter().enumerate() {
            if v == max {
                idx = i;
            }
        }
        idx
    };

    let mut nt = VecDeque::new();
    nt.push_back((start, 0));
    let mut d = vec![-1; n];
    while let Some((now, nd)) = nt.pop_front() {
        if d[now] >= 0 {
            continue;
        }
        d[now] = nd;
        for to in &t[now] {
            if d[*to] >= 0 {
                continue;
            }
            nt.push_back((*to, nd+1));
        }
    }

    let max = d.into_iter().max().unwrap();
    println!("{}", max + 1);
}