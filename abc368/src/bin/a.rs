use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}
    a.rotate_right(k);
    println!("{}", a.iter().join(" "));
}
