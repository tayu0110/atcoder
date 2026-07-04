use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize}
    if n == 1 {
        println!("1\n1");
        return;
    }
    println!("{}", n / 2);
    println!("{}", (2..=n).step_by(2).join(" "))
}
