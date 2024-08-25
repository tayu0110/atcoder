use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut ins = vec![0; n];
    for (a, b) in e {
        ins[b - 1] += 1;
        t[a - 1].push(b - 1);
    }

    let mut nt = BinaryHeap::new();
    for i in 0..n {
        if ins[i] == 0 {
            nt.push(Reverse(i));
        }
    }

    let mut res = vec![];
    while let Some(Reverse(now)) = nt.pop() {
        res.push(now + 1);
        for &to in &t[now] {
            ins[to] -= 1;

            if ins[to] == 0 {
                nt.push(Reverse(to));
            }
        }
    }

    println!("{}", res.iter().join(" "))
}
