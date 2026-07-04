use std::{cmp::Reverse, collections::BinaryHeap, hash::BuildHasherDefault, iter::from_fn};

use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n], mut b: [usize; n], mut c: [usize; n]}

    for a in [&mut a, &mut b, &mut c] {
        a.sort_unstable_by_key(|&a| Reverse(a));
    }

    let mut checked = FxHashSet::with_capacity_and_hasher(k, BuildHasherDefault::default());
    checked.insert((0, 0, 0));
    let mut nt = BinaryHeap::with_capacity(k * 2);
    let calc = |i: usize, j: usize, k: usize| -> usize { a[i] * b[j] + b[j] * c[k] + c[k] * a[i] };
    nt.push((calc(0, 0, 0), 0, 0, 0));
    println!(
        "{}",
        from_fn(move || {
            let (res, i, j, k) = nt.pop().unwrap();
            for l in 0..3 {
                let mut buf = [i, j, k];
                buf[l] += 1;
                let [i, j, k] = buf;
                if i < n && j < n && k < n && !checked.contains(&(i, j, k)) {
                    checked.insert((i, j, k));
                    nt.push((calc(i, j, k), i, j, k));
                }
            }
            Some(res)
        })
        .skip(k - 1)
        .next()
        .unwrap()
    )
}
