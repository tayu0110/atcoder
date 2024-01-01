use proconio::*;

fn main() {
    input! {n: usize, mut s: [String; n]}

    s.sort();
    s.dedup();

    println!("{}", s.len())
}
