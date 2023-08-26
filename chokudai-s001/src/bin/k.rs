use fenwick_tree::FenwickTree;
use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut prod = Modint::one();
    for i in 2..n {
        prod *= Modint::raw(i as u32);
    }

    let mut res = Modint::zero();
    let mut ft = FenwickTree::new(n + 1, 0usize);
    for (i, a) in a.into_iter().enumerate() {
        let min = a - 1 - ft.get_sum(0, a);
        res += prod * Modint::raw(min as u32);
        if n - i - 1 > 0 {
            prod /= Modint::raw((n - i - 1) as u32);
        }
        ft.add(a, 1);
    }

    println!("{}", res + Modint::one());
}
