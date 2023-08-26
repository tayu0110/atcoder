use proconio::*;

fn main() {
    input! {r: f64}
    println!("{}", r * 2.0 * std::f64::consts::PI);
}
