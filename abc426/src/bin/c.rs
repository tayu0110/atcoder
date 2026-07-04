use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut nt = (1..=n).map(|i| Reverse((i, 1))).collect::<BinaryHeap<_>>();
    let mut ret = vec![];
    for _ in 0..q {
        input! {x: usize, y: usize}
        let mut cnt = 0;
        while let Some(Reverse((ver, c))) = nt.pop() {
            if ver <= x {
                cnt += c;
            } else {
                nt.push(Reverse((ver, c)));
                break;
            }
        }

        ret.push(cnt);
        nt.push(Reverse((y, cnt)));
    }

    println!("{}", ret.iter().join("\n"))
}
