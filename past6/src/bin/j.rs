use proconio::*;

fn main() {
    input! {n: usize, c: i64, p: [(i64, i64); n]}

    let t = p
        .iter()
        .map(|&(x, y)| x * x + (c - y) * (c - y))
        .sum::<i64>();
    let s = p.iter().map(|v| v.0).sum::<i64>();
    println!("{}", t as f64 - s as f64 / n as f64 * s as f64)
}
