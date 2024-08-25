use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    println!("{}", a.windows(2).map(|v| v[0] * v[1]).join(" "))
}
