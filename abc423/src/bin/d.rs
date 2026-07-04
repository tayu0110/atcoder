use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, query: [(usize, usize, usize); n]}

    let mut cap = 0;
    let mut now = 0;
    let mut out = BinaryHeap::new();
    let mut ret = vec![];
    for (a, b, c) in query {
        now = now.max(a);
        while out.peek().filter(|Reverse((out, _))| *out <= a).is_some() {
            let Reverse((_, c)) = out.pop().unwrap();
            cap -= c;
        }

        while cap + c > k {
            let Reverse((out, c)) = out.pop().unwrap();
            now = out;
            cap -= c;
        }

        ret.push(now);
        out.push(Reverse((now + b, c)));
        cap += c;
    }

    println!("{}", ret.iter().join("\n"))
}
