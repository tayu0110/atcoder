use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}
    println!("{}", (a + b + c <= 21) as usize)
}
