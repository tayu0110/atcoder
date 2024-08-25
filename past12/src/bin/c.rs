use itertools::Itertools;
use proconio::*;

fn main() {
    input! {p: [[usize; 6]; 3]}

    let mut res = [0; 19];
    for i in 1..=6 {
        for j in 1..=6 {
            for k in 1..=6 {
                let (ni, nj, nk) = (p[0][i - 1], p[1][j - 1], p[2][k - 1]);
                res[i + j + k] += ni * nj * nk;
            }
        }
    }

    println!(
        "{}",
        res.iter().skip(1).map(|&v| v as f64 / 1000000.0).join("\n")
    )
}
