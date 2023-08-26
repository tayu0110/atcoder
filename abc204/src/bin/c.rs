use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
    }

    let mut res = 0;
    for i in 0..n {
        let mut reached = vec![false; n];
        let mut nt = VecDeque::new();
        nt.push_back(i);
        while let Some(now) = nt.pop_front() {
            if reached[now] {
                continue;
            }
            reached[now] = true;
            for &to in &t[now] {
                if !reached[to] {
                    nt.push_back(to);
                }
            }
        }

        res += reached.into_iter().filter(|&f| f).count();
    }

    println!("{}", res)
}
