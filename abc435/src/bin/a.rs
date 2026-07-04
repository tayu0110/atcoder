use proconio::*;

fn main() {
    input! {n: usize}
    println!("{}", (1..=n).sum::<usize>())
}
