use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize, usize); n]}
    println!(
        "{}",
        p.into_iter()
            .filter(|p| p.0 + 10 <= p.1)
            .map(|p| p.2)
            .sum::<usize>()
    )
}
