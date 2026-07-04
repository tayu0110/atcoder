use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    println!(
        "{}",
        a.into_iter()
            .enumerate()
            .filter(|v| v.0 % 2 == 0)
            .map(|v| v.1)
            .sum::<usize>()
    )
}
