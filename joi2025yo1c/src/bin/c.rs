use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize}
    println!(
        "{}",
        (1..=n).filter(|&i| (i % a == 0) ^ (i % b == 0)).count()
    )
}
