use proconio::*;

fn main() {
    input! {a: i32, b: i32}
    println!("{}", 3 - (a == b) as i32 * 2 - (a + b).rem_euclid(2));
}
