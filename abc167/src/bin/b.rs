use proconio::*;

fn main() {
    input! {a: i32, b: i32, _: i32, k: i32}
    if k < a {
        println!("{}", k)
    } else if a + b >= k {
        println!("{}", a)
    } else {
        println!("{}", a - (k - a - b))
    }
}
