use proconio::*;

fn main() {
    input! {n: usize, w: [i32; n]}
    println!(
        "{}",
        (1..n)
            .map(|i| (w[0..i].iter().sum::<i32>() - w[i..n].iter().sum::<i32>()).abs())
            .min()
            .unwrap()
    )
}
