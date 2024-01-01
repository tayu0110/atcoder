use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [u32; n]}

    let mut a = a.into_iter().enumerate().collect::<Vec<_>>();
    a.sort_by_key(|v| v.1);

    let mut b = a
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i, v.0))
        .collect::<Vec<_>>();
    b.sort_unstable_by_key(|v| v.1);

    println!("{}", b.into_iter().map(|v| v.0 + 1).join(" "))
}
