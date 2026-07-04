use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, d: [usize; n - 1]}

    for i in 0..n - 1 {
        let mut sum = 0;
        let mut res = vec![];
        for j in i..n - 1 {
            sum += d[j];
            res.push(sum);
        }
        println!("{}", res.iter().join(" "))
    }
}
