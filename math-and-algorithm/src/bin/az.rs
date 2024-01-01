use proconio::*;

fn main() {
    input! {n: u64}
    println!("{}", if n & 0b11 != 0 { "First" } else { "Second" })
}
