use proconio::*;

fn main() {
    input! {s: marker::Chars}
    if s[2] == s[3] && s[4] == s[5] {
        println!("Yes")
    } else {
        println!("No")
    }
}
