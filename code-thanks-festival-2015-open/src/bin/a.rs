use proconio::*;

fn main() {
    input! {a: i32, b: i32}
    println!("{}", a.abs() + (b - a).abs() + b.abs());
}
