use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], mut b: [usize; m]}

    b.sort_unstable();
    let mut a = a.into_iter().map(|a| Reverse(a)).collect::<BinaryHeap<_>>();
    let mut res = 0;
    'b: for b in b {
        while let Some(Reverse(a)) = a.pop() {
            if a < b {
                continue;
            }

            res += a;
            continue 'b;
        }

        println!("-1");
        return;
    }

    println!("{res}")
}
