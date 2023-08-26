use proconio::*;

fn main() {
    input! {a: [usize; 64]}
    println!("{}", a.into_iter().rev().fold(0usize, |s, v| (s << 1) | v))
}
