use proconio::*;

fn main() {
    input! {a: i64, b: i64, c: i64}
    if a + b + c == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
