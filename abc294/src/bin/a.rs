use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    let a = a.into_iter().filter(|v| *v % 2 == 0).collect::<Vec<_>>();
    println!("{}", a.into_iter().join(" "))
}
