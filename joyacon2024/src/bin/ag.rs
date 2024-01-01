use proconio::*;

fn main() {
    input! {mut s: marker::Bytes}

    s.sort();

    if s[0] == s[1] && s[2] == s[3] && s[1] != s[2] {
        println!("Yes")
    } else {
        println!("No")
    }
}
