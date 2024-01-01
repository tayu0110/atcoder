use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [u32; n - 1]}

    let mut res = vec![0; n];
    for (i, a) in a.into_iter().enumerate().rev() {
        res[a as usize - 1] += res[i + 1] + 1;
    }

    println!("{}", res.iter().join(" "))
}
