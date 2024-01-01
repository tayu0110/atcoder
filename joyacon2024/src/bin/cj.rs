use proconio::*;

fn main() {
    input! {n: usize, p: [(f64, String); n]}

    println!(
        "{}",
        p.into_iter()
            .map(|(x, u)| if u == "JPY" { x } else { x * 380000. })
            .sum::<f64>()
    )
}
