use proconio::*;

fn main() {
    input! {r1: f64, r2: f64}

    println!("{}", 1.0 / (1.0 / r1 + 1.0 / r2))
}
