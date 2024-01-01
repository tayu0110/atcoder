use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let &max = a.iter().max().unwrap();
    println!(
        "{}",
        a.into_iter()
            .map(|a| {
                let t = a * 10_000_000_000 / max;
                t / 10 + (t % 10 > 4) as usize
            })
            .join(" ")
    )
}
