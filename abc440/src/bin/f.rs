use std::fmt::Write as _;

use fenwick_tree::{Addition, FenwickTree};
use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut p: [(usize, usize); n]}

    let mut two = p.iter().filter(|p| p.1 == 2).count();
    let mut ft = FenwickTree::<Addition<usize>>::new(1000001);
    let mut sum = FenwickTree::<Addition<usize>>::new(1000001);
    for &(a, _) in &p {
        ft.add(a, 1);
        sum.add(a, a);
    }

    let mut buf = String::new();
    for _ in 0..q {
        input! {w: usize, x: usize, y: usize}
        if p[w - 1].1 == 2 {
            two -= 1;
        }
        if y == 2 {
            two += 1;
        }
        ft.add(p[w - 1].0, 1usize.wrapping_neg());
        sum.add(p[w - 1].0, p[w - 1].0.wrapping_neg());
        p[w - 1] = (x, y);

        let pos = ft.partition_point(|&b| n - b < two);
        let mut ret = sum.fold(..);
    }
}
