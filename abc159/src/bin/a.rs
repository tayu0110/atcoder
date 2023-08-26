use proconio::*;

fn main() {
    input! {n: usize, m: usize}
    println!(
        "{}",
        n * n.saturating_sub(1) / 2 + m * m.saturating_sub(1) / 2
    )
}
