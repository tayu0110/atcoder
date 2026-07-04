use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut a: [marker::Chars; n]}

    let mut res = vec![vec!['.'; n]; n];
    for i in 0..n {
        for j in 0..n {
            let c = i.min(n - 1 - i).min(j).min(n - 1 - j);
            let c = (c % 4 + 1) % 4;
            let (mut ni, mut nj) = (i, j);
            for _ in 0..c {
                (ni, nj) = (nj, n - 1 - ni);
            }
            res[ni][nj] = a[i][j];
        }
    }

    for a in res {
        println!("{}", a.iter().join(""))
    }
}
