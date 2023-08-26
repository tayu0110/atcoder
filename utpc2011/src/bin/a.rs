use proconio::*;

fn main() {
    input! {m: usize, n: usize, a: [[usize; n]; m]}
    println!(
        "{}",
        a.iter().map(|v| v.iter().sum::<usize>()).max().unwrap()
    )
}
