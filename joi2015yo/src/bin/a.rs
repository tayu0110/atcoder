use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize, p: usize}
    println!("{}", (a * p).min(b + (p.saturating_sub(c) * d)))
}
