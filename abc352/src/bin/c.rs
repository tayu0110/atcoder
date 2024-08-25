use proconio::*;

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}

    p.sort_unstable_by_key(|(a, b)| b - a);
    let (_, b) = p.pop().unwrap();
    println!("{}", p.into_iter().map(|v| v.0).sum::<usize>() + b)
}
