use proconio::*;

fn main() {
    input! {n: usize, d: i64, p: [(i64, i64); n]}
    println!(
        "{}",
        p.into_iter()
            .map(|(x, y)| x * x + y * y)
            .filter(|&v| v <= d * d)
            .count()
    )
}
