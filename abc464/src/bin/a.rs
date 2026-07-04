use proconio::*;

fn main() {
    input! {s: marker::Bytes}
    if s.iter().filter(|&&b| b == b'E').count() * 2 > s.len() {
        println!("East")
    } else {
        println!("West")
    }
}
