use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut a = a.into_iter().enumerate().collect::<Vec<_>>();
    a.sort_by_key(|a| a.1);

    println!("{}", a.into_iter().map(|(i, _)| i + 1).join(" "))
}
