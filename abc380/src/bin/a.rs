use proconio::*;

fn main() {
    input! {mut n: marker::Bytes}
    n.sort();
    if n == b"122333" {
        println!("Yes")
    } else {
        println!("No")
    }
}
