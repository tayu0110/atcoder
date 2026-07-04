use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut a: [usize; 3]}
    a.sort();
    a.reverse();
    println!("{}", a.iter().join(""))
}
