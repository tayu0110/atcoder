use num::integer::Roots;
use proconio::*;

fn main() {
    input! {n: usize}

    let sqrt = n.sqrt();
    println!("{}", sqrt.pow(2))
}
