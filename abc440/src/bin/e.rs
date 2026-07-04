use std::{cmp::Reverse, collections::BinaryHeap, fmt::Write as _};

use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, k: i64, x: i64, mut a: [i64; n]}

    a.sort_unstable_by_key(|&a| Reverse(a));
    let max = a[0];
    if n == 1 {
        println!("{}", max * k * x);
        return;
    }

    let mut nt = BinaryHeap::new();
    let mut tmp = vec![0; n];
    tmp[0] = k;
    nt.push((max * k, tmp));
    let mut buf = String::new();
    let mut checked = FxHashSet::default();
    for _ in 0..x {
        while let Some((ret, t)) = nt.pop() {
            if !checked.insert(t.clone()) {
                continue;
            }
            writeln!(buf, "{ret}").unwrap();

            for i in 0..n - 1 {
                if t[i] > 0 {
                    let mut u = t.clone();
                    u[i + 1] += 1;
                    u[i] -= 1;
                    nt.push((ret + (a[i + 1] - a[i]), u));
                }
            }
            break;
        }
    }
    print!("{buf}")
}
