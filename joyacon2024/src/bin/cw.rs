use proconio::*;

fn main() {
    input! {n: usize, v: [usize; n], c: [usize; n]}

    println!(
        "{}",
        v.into_iter()
            .zip(c)
            .map(|(v, c)| v.saturating_sub(c))
            .sum::<usize>()
    )
}
