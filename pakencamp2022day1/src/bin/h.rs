use std::{cmp::Reverse, collections::BinaryHeap, iter::once};

use cpio::*;

fn main() {
    scan!(n: usize, q: usize, mut a: [usize; n], query: [(u8, usize, usize); q]);

    let mut rem = a
        .iter()
        .copied()
        .enumerate()
        .map(|(i, a)| Reverse((a, i)))
        .collect::<BinaryHeap<_>>();
    let mut base = 0;
    let mut res = vec![];
    let mut dropped = vec![false; n];
    for (ty, x, y) in query {
        if ty == 1 {
            a[x - 1] += y;
            base += y;
        } else {
            a[x - 1] = a[x - 1].saturating_sub(y);
        }
        rem.push(Reverse((a[x - 1], x - 1)));

        while let Some(&Reverse((na, i))) = rem.peek() {
            if a[i] != na || dropped[i] {
                rem.pop();
                continue;
            }

            if na > base {
                break;
            }

            rem.pop();
            res.push(i + 1);
            dropped[i] = true;
        }

        res.sort_unstable();
        putln!(once(res.len()).chain(res.iter().cloned()), @sep = " ");
        res.clear();
    }
}
