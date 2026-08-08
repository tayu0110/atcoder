use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n], q: [usize; n]}

    println!(
        "{}",
        (1..=n).permutations(n).filter(|r| &p < r && r < &q).count()
    )
}
