use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); n]}

    let mut nt = p
        .into_iter()
        .map(|(l, r)| Reverse((r, l)))
        .collect::<BinaryHeap<Reverse<(usize, usize)>>>();

    let mut res = 0;
    'b: for i in 1..=m {
        while let Some(&Reverse((r, l))) = nt.peek() {
            if l < i {
                nt.pop();
                continue;
            }

            res += r - i;
            continue 'b;
        }

        res += m - i + 1;
    }

    println!("{res}")
}
