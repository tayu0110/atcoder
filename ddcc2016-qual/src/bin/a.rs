use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}
    println!("{}", (c * b) as f64 / a as f64)
}
