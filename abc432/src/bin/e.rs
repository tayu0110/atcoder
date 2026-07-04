use std::fmt::Write as _;

use fenwick_tree::{Addition, FenwickTree};
use proconio::*;

const N: usize = 500001;

fn main() {
    input! {n: usize, q: usize, mut a: [usize; n]}

    let mut cnt = FenwickTree::<Addition<usize>>::new(N);
    let mut sum = FenwickTree::<Addition<usize>>::new(N);
    for &a in &a {
        cnt.add(a, 1);
        sum.add(a, a);
    }

    let mut buf = String::new();
    for _ in 0..q {
        input! {ty: usize}
        if ty == 1 {
            input! {x: usize, y: usize}
            cnt.add(a[x - 1], usize::MAX);
            sum.add(a[x - 1], a[x - 1].wrapping_neg());

            a[x - 1] = y;
            cnt.add(y, 1);
            sum.add(y, y);
        } else {
            input! {l: usize, r: usize}

            if l <= r {
                writeln!(
                    buf,
                    "{}",
                    sum.fold(l..=r) + cnt.fold(..l) * l + cnt.fold(r + 1..) * r
                )
                .unwrap();
            } else {
                writeln!(buf, "{}", l * n).unwrap();
            }
        }
    }

    print!("{buf}")
}
