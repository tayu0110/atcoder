use proconio::*;

fn main() {
    input! {mut x: usize}
    x -= 1;
    println!("{}", (x % 7 == 1) as usize)
}
