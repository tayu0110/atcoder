use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let mut res = vec![m; n];
    for a in a {
        res[a - 1] -= 1;
    }

    println!("{}", res.into_iter().join("\n"))
}
