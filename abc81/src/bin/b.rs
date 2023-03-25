use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    println!(
        "{}",
        a.into_iter().map(|v| v.trailing_zeros()).min().unwrap()
    )
}
