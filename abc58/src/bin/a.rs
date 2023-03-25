use proconio::*;

fn main() {
    input! {a: i32, b: i32, c: i32}
    if b - a == c - b {
        println!("YES")
    } else {
        println!("NO")
    }
}
