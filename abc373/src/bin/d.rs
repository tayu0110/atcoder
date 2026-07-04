use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

const MAX: i64 = 10i64.pow(18);
const MIN: i64 = -MAX;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, i64); m]}

    let mut t = vec![vec![]; n];
    for (u, v, w) in e {
        t[u - 1].push((v - 1, w));
        t[v - 1].push((u - 1, -w));
    }

    let mut nt = VecDeque::new();
    let mut res = vec![i64::MAX; n];
    for i in 0..n {
        if res[i] == i64::MAX {
            res[i] = 0;
            nt.push_back((i, 0));
            while let Some((now, d)) = nt.pop_front() {
                for &(to, w) in &t[now] {
                    if res[to] == i64::MAX {
                        res[to] = d + w;
                        nt.push_back((to, d + w));
                    } else {
                        assert_eq!(d + w, res[to]);
                    }
                }
            }
        }
    }

    if *res.iter().min().unwrap() < MIN {
        let diff = MIN - *res.iter().min().unwrap();
        res.iter_mut().for_each(|r| *r += diff);
    }
    if *res.iter().max().unwrap() > MAX {
        let diff = *res.iter().max().unwrap() - MAX;
        res.iter_mut().for_each(|r| *r -= diff);
    }

    assert!(*res.iter().min().unwrap() >= MIN);
    assert!(*res.iter().max().unwrap() <= MAX);

    println!("{}", res.iter().join(" "))
}
