use math::MathInt;
use proconio::*;

fn main() {
    input! {n: i64}

    let pi = n.count_primes() as i64;
    println!("{}", pi - (n - pi));
}
