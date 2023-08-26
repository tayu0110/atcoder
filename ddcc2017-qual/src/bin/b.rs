use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize}
    println!("{}", c * 12 + b * 144 + a * 1728 + d)
}
