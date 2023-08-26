use matrix::Matrix;
use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, m: u32}

    let mut mat = Matrix::new(2, 2);
    mat.set(0, 0, Modint::raw(m - 2));
    mat.set(0, 1, Modint::raw(m - 1));
    mat.set(1, 0, Modint::one());

    let mut t = Matrix::new(2, 1);
    t.set(0, 0, Modint::zero());
    t.set(1, 0, Modint::raw(m));

    let mat = mat.pow(n - 1);
    let res = mat.mul(&t);
    println!("{}", res.get(0, 0));
}
