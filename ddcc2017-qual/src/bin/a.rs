use proconio::*;

fn main() {
    input! {s: marker::Chars}
    if s[0] == s[1] && s[1] != s[2] && s[2] == s[3] {
        println!("Yes")
    } else {
        println!("No")
    }
}
