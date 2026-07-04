use fenwick_tree::{AbelianGroup, FenwickTree};
use proconio::*;

struct T;
impl AbelianGroup for T {
    type G = usize;
    fn e() -> Self::G {
        0
    }
    fn op(l: &Self::G, r: &Self::G) -> Self::G {
        l.wrapping_add(*r)
    }
    fn inv(g: &Self::G) -> Self::G {
        g.wrapping_neg()
    }
}

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut inversion = 0;
    let mut bit = FenwickTree::<T>::new(m + 1);
    let mut increase = vec![0; m + 1];
    for i in 0..n {
        inversion += bit.fold(a[i] + 1..);
        increase[a[i]] += i - bit.get(a[i]).unwrap();
        bit.add(a[i], 1);
    }
    let mut count = vec![0; m + 1];
    let mut decrease = vec![0; m + 1];
    for i in (0..n).rev() {
        decrease[a[i]] += n - 1 - i - count[a[i]];
        count[a[i]] += 1;
    }

    println!("{}", inversion);
    for k in 1..m {
        inversion += increase[m - k];
        inversion -= decrease[m - k];
        println!("{}", inversion);
    }
}
