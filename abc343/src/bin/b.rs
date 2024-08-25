use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [[u8; n]; n]}

    for i in 0..n {
        let mut res = vec![];
        for j in 0..n {
            if a[i][j] == 1 {
                res.push(j + 1);
            }
        }

        println!("{}", res.iter().join(" "))
    }
}
