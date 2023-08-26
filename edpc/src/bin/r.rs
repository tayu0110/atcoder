use matrix::Matrix;
use proconio::input;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, k: usize, a: [[i32; n]; n]}

    let matrix = Matrix::from(a);

    let res: Matrix<StaticModint<Mod1000000007>> = matrix.pow(k);

    let mut sum = Modint::zero();
    for i in 0..n {
        for j in 0..n {
            sum += res.get(i, j);
        }
    }

    println!("{}", sum);
}
