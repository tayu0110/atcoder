use fenwick_tree::{Addition, FenwickTree};
use proconio::*;

fn main() {
    input! {n: usize, b: [usize; n]}

    let mut ret = n * (n + 1) / 2;
    let mut ft = FenwickTree::<Addition<usize>>::new(n);
    let mut pos = vec![0; n];
    for (i, b) in b.into_iter().enumerate() {
        pos[b - 1] = i;
    }

    for p in pos {
        ret += ft.fold(..p) * ft.fold(p..);
        ft.add(p, 1);
    }

    println!("{}", ret);
}
