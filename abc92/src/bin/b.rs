use proconio::*;

fn main() {
    input! {n: usize, d: usize, x: usize, a: [usize; n]}
    println!(
        "{}",
        x + a.into_iter().map(|a| (d + a - 1) / a).sum::<usize>()
    )
}
