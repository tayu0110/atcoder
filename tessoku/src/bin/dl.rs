use proconio::*;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    input! {n: usize, d: u32, mut p: [(u32, u32); n]}
    p.sort_unstable();

    let mut nt = VecDeque::with_capacity(n);
    nt.extend(p);
    let mut max = BinaryHeap::new();
    let mut res = 0;
    for i in 1..=d {
        while !nt.is_empty() && nt.front().unwrap().0 == i {
            max.push(nt.pop_front().unwrap().1);
        }

        if let Some(m) = max.pop() {
            res += m;
        }
    }

    println!("{res}")
}
