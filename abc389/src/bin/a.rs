use proconio::*;

fn main() {
    input! {s: marker::Bytes}
    println!("{}", (s[0] - b'0') * (s[2] - b'0'))
}
