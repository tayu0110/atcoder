use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut p: [usize; n]}
    p.sort();
    println!("{}", p[..k].iter().sum::<usize>())
}
