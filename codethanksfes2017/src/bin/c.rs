use proconio::*;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize); n]}
    let mut nt = p.into_iter().map(Reverse).collect::<BinaryHeap<_>>();

    let mut res = 0;
    for _ in 0..k {
        let Reverse((a, b)) = nt.pop().unwrap();
        res += a;
        nt.push(Reverse((a + b, b)));
    }

    println!("{}", res)
}
