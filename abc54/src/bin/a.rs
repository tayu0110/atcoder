use std::cmp::Ordering;

use proconio::*;

fn main() {
    input! {mut a: usize, mut b: usize}

    if a == 1 {
        a += 13;
    }
    if b == 1 {
        b += 13;
    }

    match a.cmp(&b) {
        Ordering::Greater => println!("Alice"),
        Ordering::Equal => println!("Draw"),
        Ordering::Less => println!("Bob"),
    }
}
