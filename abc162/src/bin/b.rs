use proconio::*;

fn main() {
    input! {n: usize}
    println!(
        "{}",
        (1..=n).filter(|&v| v % 3 != 0 && v % 5 != 0).sum::<usize>()
    )
}
