use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}

    println!("{}", a * b % 1000000007 * c % 1000000007)
}
