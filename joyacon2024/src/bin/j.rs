use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n - 1]}

    let mut res = vec![0; n];
    for i in (1..n).rev() {
        res[a[i - 1] - 1] += 1;
    }

    println!("{}", res.iter().join("\n"))
}
