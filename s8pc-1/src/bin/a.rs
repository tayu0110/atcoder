use proconio::*;

fn main() {
    input! {n: usize}
    println!("{}", (18 * 60) as f64 / (n - 1) as f64)
}
