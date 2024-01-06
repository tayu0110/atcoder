use itertools::Itertools;
use matrix::Matrix;
use proconio::input;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, k: usize, a: [[i32; n]; n]}

    let matrix = Matrix::from_vec_as_shape(
        n,
        n,
        a.into_iter()
            .flatten()
            .map(|a| Modint::new_signed(a as i64))
            .collect_vec(),
    );

    let res: Matrix<StaticModint<Mod1000000007>> = matrix.pow(k as u64);

    let mut sum = Modint::zero();
    for i in 0..n {
        for j in 0..n {
            sum += *res.get(i, j);
        }
    }

    println!("{}", sum);
}
