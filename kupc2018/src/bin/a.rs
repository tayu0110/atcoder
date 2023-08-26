use proconio::*;

fn main() {
    input! {n: usize, s: [usize; n], a: [usize; n]}
    println!(
        "{}",
        s.into_iter().zip(a).map(|(s, a)| s * a).max().unwrap()
    )
}
