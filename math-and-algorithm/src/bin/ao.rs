use std::collections::{BTreeSet, VecDeque};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut color = vec![-1i8; n];
    let mut nt = VecDeque::new();
    let mut remain = (0..n).collect::<BTreeSet<_>>();
    while let Some(top) = remain.pop_first() {
        color[top] = 0;
        nt.push_back(top);
        while let Some(now) = nt.pop_front() {
            for &to in &t[now] {
                if color[to] < 0 {
                    color[to] = 1 ^ color[now];
                    nt.push_back(to);
                    remain.remove(&to);
                } else if color[to] ^ 1 != color[now] {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes")
}
