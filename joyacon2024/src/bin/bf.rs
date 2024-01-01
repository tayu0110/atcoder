use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}
    println!("{}", a * b * 2 + b * c * 2 + c * a * 2)
}
