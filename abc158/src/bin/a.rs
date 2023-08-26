use proconio::*;

fn main() {
    input! {s: marker::Chars}
    if s.iter().all(|&c| c == s[0]) {
        println!("No")
    } else {
        println!("Yes")
    }
}
