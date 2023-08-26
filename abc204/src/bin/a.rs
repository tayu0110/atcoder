use proconio::*;

fn main() {
    input! {x: i32, y: i32}
    println!("{}", (3 - x - y).rem_euclid(3))
}
