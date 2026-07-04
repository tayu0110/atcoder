use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}
    println!(
        "{}",
        a.into_iter().fold(0, |s, v| s + b
            .iter()
            .map(|&b| (b + v) * b.max(v))
            .sum::<usize>())
    )
}
