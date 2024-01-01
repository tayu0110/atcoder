use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    println!("{}", 1. / a.into_iter().map(|a| 1. / a as f64).sum::<f64>())
}
