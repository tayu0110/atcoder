use proconio::*;

fn main() {
    input! {n: i32}
    println!("{}", 2i32.pow(n as u32) - 2 * n)
}
