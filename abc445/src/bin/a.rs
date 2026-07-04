use proconio::*;

fn main() {
    input! {s: marker::Bytes}
    if s[0] == *s.last().unwrap() {
        println!("Yes")
    } else {
        println!("No")
    }
}
