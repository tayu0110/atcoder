use math::MathInt;
use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize}
    println!(
        "{}",
        (0..n).map(|i| a * i + b).filter(|&a| a.is_prime()).count()
    );
}
