use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n]}
    println!(
        "{}",
        (1..=1000)
            .filter(|&i| a
                .iter()
                .cloned()
                .zip(b.iter().cloned())
                .all(|(l, r)| l <= i && i <= r))
            .count()
    )
}
