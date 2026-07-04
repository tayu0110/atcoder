use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    use std::fmt::Write as _;
    let mut buf = String::new();
    for _ in 0..t {
        input! {n: usize, m: usize, x: usize, y: usize, e: [(usize, usize); m]}

        let mut t = vec![vec![]; n];
        for (u, v) in e {
            t[u - 1].push(v - 1);
            t[v - 1].push(u - 1);
        }

        let mut nt = BinaryHeap::new();
        nt.push(Reverse((vec![x], x - 1)));
        let mut checked = vec![false; n];
        while let Some(Reverse((seq, now))) = nt.pop() {
            if checked[now] {
                continue;
            }
            checked[now] = true;
            if now == y - 1 {
                writeln!(buf, "{}", seq.iter().join(" ")).ok();
                break;
            }

            for &to in &t[now] {
                if checked[to] {
                    continue;
                }

                let mut seq = seq.clone();
                seq.push(to + 1);
                nt.push(Reverse((seq, to)));
            }
        }
    }

    print!("{buf}")
}
