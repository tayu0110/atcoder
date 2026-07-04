use proconio::*;

fn main() {
    input! {_n: usize, l: usize, r: usize, s: marker::Bytes}
    if s[l - 1..r].iter().all(|c| c == &b'o') {
        println!("Yes")
    } else {
        println!("No")
    }
}
