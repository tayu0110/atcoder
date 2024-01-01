use num::integer::Roots;
use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    let s: usize = format!("{a}{b}").parse().unwrap();
    let q = s.sqrt();

    if q * q == s {
        println!("Yes")
    } else {
        println!("No")
    }
}
