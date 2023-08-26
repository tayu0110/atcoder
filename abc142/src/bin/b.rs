use proconio::*;

fn main() {
    input! {n: usize, k: usize, h: [usize; n]}
    println!("{}", h.into_iter().filter(|&h| h >= k).count())
}
