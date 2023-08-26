use std::collections::BinaryHeap;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut par: [usize; n-1], p: [(usize, i32); m]}

    let mut child = vec![vec![]; n];
    for (i, &par) in par.iter().enumerate() {
        child[par - 1].push(i + 1);
    }

    let mut nt = BinaryHeap::new();
    for (x, y) in p {
        nt.push((y, x - 1));
    }

    let mut rem = vec![-1; n];
    while let Some((r, now)) = nt.pop() {
        if rem[now] >= 0 {
            continue;
        }
        rem[now] = r;

        if r > 0 {
            for &c in &child[now] {
                nt.push((r - 1, c));
            }
        }
    }
    // eprintln!("rem: {rem:?}");

    println!("{}", rem.iter().filter(|&&f| f >= 0).count())
}
