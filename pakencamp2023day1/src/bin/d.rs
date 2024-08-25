use proconio::*;

fn main() {
    input! {k: i64, x1: i64, y1: i64, x2: i64, y2: i64}
    let p = (x2 - x1).abs();
    let q = (y2 - y1).abs();

    let s = p + q;
    let t = (p - q).abs();
    println!("{}", (s + 2 * k - 1) / (2 * k) + (t + 2 * k - 1) / (2 * k));
}
