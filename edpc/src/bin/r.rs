use matrix::Matrix;
use modint::{Mint, Mod1000000007};
use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [[i32; n]; n]}

    let matrix = Matrix::<Mod1000000007>::from(a);

    let res: Matrix<Mod1000000007> = matrix.pow(k);

    let mut sum = Mint::zero();
    for i in 0..n {
        for j in 0..n {
            sum += res.get(i, j);
        }
    }

    println!("{}", sum);
}
