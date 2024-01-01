use proconio::*;

fn main() {
    input! {n: usize, x: usize, s: [usize; n]}

    println!("{}", s.into_iter().filter(|&s| s <= x).sum::<usize>())
}
