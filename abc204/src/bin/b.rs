use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    println!(
        "{}",
        a.into_iter().map(|v| v.saturating_sub(10)).sum::<usize>()
    )
}
