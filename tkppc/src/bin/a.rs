use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}
    p.into_iter().for_each(|(a, b)| println!("{}", a + b))
}
