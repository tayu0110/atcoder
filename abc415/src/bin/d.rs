use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {mut n: usize, m: usize, e: [(usize, usize); m]}

    let mut nt = e
        .into_iter()
        .map(|(a, b)| Reverse((a - b, a)))
        .collect::<BinaryHeap<_>>();

    let mut res = 0usize;
    while let Some(Reverse((d, a))) = nt.pop() {
        let div = n.saturating_sub(a - d) / d;
        res += div;
        n -= div * d;
    }

    println!("{res}")
}
