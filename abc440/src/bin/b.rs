use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, t: [usize; n]}

    let mut t = t.into_iter().enumerate().collect::<Vec<_>>();
    t.sort_unstable_by_key(|t| t.1);
    println!("{}", t.iter().map(|t| t.0 + 1).take(3).join(" "));
}
