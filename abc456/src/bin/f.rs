use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, k: usize, a: [usize; n]}
        if k == 1 {
            println!("{}", a.iter().min().unwrap());
            continue;
        }

        let mut nt = BinaryHeap::new();
        for i in (0..n).take_while(|&i| i + k <= n) {
            nt.push(Reverse((a[i], 1, i)));
        }

        while let Some(Reverse((cost, dist, i))) = nt.pop() {
            if dist >= k {
                println!("{cost}");
                break;
            }
            if i + 1 < n {
                nt.push(Reverse((cost + a[i + 1], dist + 1, i + 1)));
            }
            if i + 2 < n {
                nt.push(Reverse((cost + a[i + 2], dist + 2, i + 2)));
            }
        }
    }
}
