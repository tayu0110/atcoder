use matrix::Matrix;
use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize}

    let mat = Matrix::<Modint>::from(vec![
        vec![Modint::one(), Modint::one()],
        vec![Modint::one(), Modint::zero()],
    ])
    .pow(n as u64);

    let res = mat * Matrix::from(vec![vec![Modint::zero()], vec![Modint::one()]]);
    println!("{}", res.get(0, 0));
}
