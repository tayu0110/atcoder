use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}
    println!("{}", p.into_iter().filter(|(a, b)| a < b).count())
}
