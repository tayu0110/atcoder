use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut a: [usize; 3]}
    a.swap(0, 1);
    a.swap(0, 2);
    println!("{}", a.iter().join(" "))
}
