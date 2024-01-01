use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize, q: [(usize, usize); t]}
    println!(
        "{}",
        q.into_iter()
            .map(|(a, s)| {
                if s < 2 * a || a & (s - 2 * a) != 0 {
                    "No"
                } else {
                    "Yes"
                }
            })
            .join("\n")
    )
}
