use proconio::*;

fn main() {
    input! {mut s: marker::Bytes}
    s.sort();
    if &s == &b"ABC" {
        println!("Yes")
    } else {
        println!("No")
    }
}
