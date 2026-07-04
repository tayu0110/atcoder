use proconio::*;

fn main() {
    input! {n: usize}
    println!("{}", (n * (n + 1) / 2 % 1000_000_007).pow(2) % 1000_000_007);
}
