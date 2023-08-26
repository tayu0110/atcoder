use proconio::*;

fn main() {
    input! {a: usize, b: usize}
    println!("{}", (b * a) as f64 / 100.0)
}
