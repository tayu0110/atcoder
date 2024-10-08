use itertools::Itertools;
use math::miller_rabin_test;
use proconio::*;

fn main() {
    input! {q: usize, x: [u64; q]}

    println!(
        "{}",
        x.into_iter()
            .map(miller_rabin_test)
            .map(|f| if f { "Yes" } else { "No" })
            .join("\n")
    )
}
