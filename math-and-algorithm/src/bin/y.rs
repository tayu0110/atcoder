use proconio::*;

fn main() {
    input! {n: usize, a: [f32; n], b: [f32; n]}
    println!(
        "{}",
        a.into_iter()
            .zip(b)
            .map(|(a, b)| (a + b * 2.) / 3.)
            .sum::<f32>()
    )
}
