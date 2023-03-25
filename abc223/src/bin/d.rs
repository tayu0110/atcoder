#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut in_e = vec![0; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        in_e[b-1] += 1;
    }

    let mut nt = std::collections::BinaryHeap::new();
    for i in 0..n {
        if in_e[i] == 0 {
            nt.push(std::cmp::Reverse(i));
        }
    }

    let mut res = vec![];
    while let Some(std::cmp::Reverse(now)) = nt.pop() {
        res.push(now+1);
        
        for to in &t[now] {
            in_e[*to] -= 1;
            if in_e[*to] == 0 {
                nt.push(std::cmp::Reverse(*to));
            }
        }
    }

    if res.len() != n {
        println!("-1");
    } else {
        println!("{}", res.iter().join(" "));
    }
}
