use proconio::*;

fn main() {
    input! {n: usize}

    println!(
        "{}",
        (1..=n).filter(|&v| v % 2 == 1).count() as f64 / n as f64
    )
}
