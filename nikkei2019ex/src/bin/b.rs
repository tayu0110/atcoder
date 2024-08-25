use proconio::*;

fn main() {
    input! {n: usize}
    println!("{}", (0..).take_while(|c| c * c <= n).last().unwrap())
}
