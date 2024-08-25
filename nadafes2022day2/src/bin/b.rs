use proconio::*;

fn main() {
    input! {n: usize, a: [i64; n], b: [i64; n]}

    println!(
        "{}",
        a.iter()
            .copied()
            .zip(b.iter().copied())
            .map(|(a, b)| a * b)
            .min()
            .unwrap()
            .min(a.into_iter().max().unwrap() * b.into_iter().min().unwrap())
    )
}
