use proconio::*;

fn main() {
    input! {n: usize, p: [(f64, f64, f64, f64, f64); n]}

    println!(
        "{}",
        p.into_iter()
            .map(|(a, b, c, d, e)| a + b + c + d + e * 110. / 900.)
            .fold(0.0, f64::max)
    )
}
