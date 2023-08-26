use proconio::*;

fn main() {
    input! {a: i64, b: i64, c: i64, d: i64}
    println!("{}", (a * c).max(a * d).max(b * c).max(b * d))
}
