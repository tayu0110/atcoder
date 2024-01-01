use proconio::*;

fn main() {
    input! {n: usize, query: [(usize, usize); n]}
    println!(
        "{}",
        query
            .into_iter()
            .map(|(p, q)| q as f64 / p as f64)
            .sum::<f64>()
    )
}
