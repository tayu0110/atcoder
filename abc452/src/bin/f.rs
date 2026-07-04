use std::collections::BTreeSet;

use fenwick_tree::{Addition, FenwickTree};
use proconio::*;

fn main() {
    input! {n: usize, k: usize, p: [usize; n]}

    let mut memo = vec![1usize; n];
    for i in (0..n - 1).rev() {
        if p[i] < p[i + 1] {
            memo[i] = memo[i + 1] + 1;
        }
    }

    let mut ft = FenwickTree::<Addition<usize>>::new(n + 1);
    let (mut l, mut r) = (0, 0);
    let mut set = BTreeSet::new();
    let mut rev = 0;
    let mut ret = 0;
    while l < n {
        while r < n && rev + ft.fold(p[r]..) < k {
            set.insert(p[r]);
            rev += ft.fold(p[r]..);
            ft.add(p[r], 1);
            r += 1;
        }

        if r < n && rev + ft.fold(p[r]..) == k {
            if r == n - 1 {
                ret += 1;
            } else {
                let max = *set.last().unwrap_or(&0);
                if p[r + 1] >= max {
                    ret += memo[r];
                } else {
                    ret += 1;
                }
            }
        }

        if l < r {
            set.remove(&p[l]);
            ft.add(p[l], 1usize.wrapping_neg());
            rev -= ft.fold(..p[l]);
        } else {
            r += 1;
        }
        l += 1;
    }

    println!("{ret}")
}
