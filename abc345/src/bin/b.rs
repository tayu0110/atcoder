use proconio::*;

fn main() {
    input! {x: i64}
    if x >= 0 {
        println!("{}", (x + 9) / 10)
    } else {
        println!("{}", x / 10)
    }
}
