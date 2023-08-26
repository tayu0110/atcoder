use proconio::*;

fn main() {
    input! {n: usize, p: usize, q: usize, d: [usize; n]}
    println!("{}", p.min(q + *d.iter().min().unwrap()))
}
