use proconio::*;

fn main() {
    input! {s: marker::Chars}

    if s.windows(2).any(|v| v[0] != v[1]) {
        println!("Yes")
    } else {
        println!("No")
    }
}
