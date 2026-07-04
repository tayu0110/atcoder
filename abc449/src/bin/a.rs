use proconio::*;

fn main() {
    input! {d: f64}
    println!("{}", d * d * std::f64::consts::PI / 4.)
}
