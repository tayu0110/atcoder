use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {n: usize, p: [(usize, usize); n]};

    let mut t = vec![vec![]; n];
    let mut usable = vec![false; n];
    let mut nt = VecDeque::new();
    for (i, &(a, b)) in p.iter().enumerate() {
        t[a-1].push(i);
        t[b-1].push(i);
        if a == i+1 || b == i+1 {
            usable[i] = true;
            nt.push_back(i);
        }
    }

    let mut res = vec![];
    while let Some(now) = nt.pop_front() {
        res.push(now);
        for i in &t[now] {
            if usable[*i] {
                continue;
            }
            usable[*i] = true;
            nt.push_back(*i);
        }
    }

    res.reverse();
    if res.len() != n {
        println!("-1");
    } else {
        for v in res {
            println!("{}", v+1);
        }
    }
}