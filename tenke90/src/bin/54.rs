use std::{collections::BinaryHeap, cmp::Reverse};

use proconio::input;

fn main() {
    input! {n: usize, m: usize};

    let mut t = vec![vec![]; m];
    let mut man = vec![vec![]; n];
    for i in 0..m {
        input! {k: usize};
        for _ in 0..k {
            input! {r: usize};
            let r = r-1;
            t[i].push(r);
            man[r].push(i);
        }
    }

    let mut res = vec![-1; n];

    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 0)));
    while !nt.is_empty() {
        let Reverse((level, now)) = nt.pop().unwrap();
        if res[now] >= 0 {
            continue;
        }
        res[now] = level;
        for v in &man[now] {
            let mut buf = vec![];
            for to in &t[*v] {
                if res[*to] >= 0 {
                    continue;
                }
                nt.push(Reverse((level+1, *to)));
            }
            std::mem::swap(&mut t[*v], &mut buf);
        }
    }

    for v in res {
        println!("{}", v);
    }
}