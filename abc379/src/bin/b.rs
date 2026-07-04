use proconio::*;

fn main() {
    input! {_: usize, k: usize, s: String}
    println!("{}", s.split('X').map(|s| s.len() / k).sum::<usize>())
}
