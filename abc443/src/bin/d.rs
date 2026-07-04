use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut r: [usize; n]}

        let mut decided = vec![false; n];
        let mut nt = BinaryHeap::new();
        for (i, &r) in r.iter().enumerate() {
            nt.push(Reverse((r, i)));
        }

        let mut ret = 0;
        while let Some(Reverse((_, i))) = nt.pop() {
            if decided[i] {
                continue;
            }
            decided[i] = true;

            if i > 0 && !decided[i - 1] {
                if r[i - 1].abs_diff(r[i]) > 1 {
                    nt.push(Reverse((r[i] + 1, i - 1)));
                    ret += r[i - 1].abs_diff(r[i]) - 1;
                    r[i - 1] = r[i] + 1;
                }
            }
            if i + 1 < n && !decided[i + 1] {
                if r[i + 1].abs_diff(r[i]) > 1 {
                    nt.push(Reverse((r[i] + 1, i + 1)));
                    ret += r[i + 1].abs_diff(r[i]) - 1;
                    r[i + 1] = r[i] + 1;
                }
            }
        }

        println!("{ret}")
    }
}
