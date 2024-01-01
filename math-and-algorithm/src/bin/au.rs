use matrix::Matrix;
use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize}

    let mat = Matrix::<Modint>::from(vec![
        vec![Modint::raw(2), Modint::one()],
        vec![Modint::one(), Modint::zero()],
    ])
    .pow(n as u64 - 1);

    let res = mat * Matrix::from(vec![vec![Modint::one()], vec![Modint::one()]]);
    println!("{}", res.get(1, 0))
}
