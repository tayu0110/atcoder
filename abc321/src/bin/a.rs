use proconio::*;

fn main() {
    input! {s: marker::Chars}

    if s.windows(2).all(|v| v[0] > v[1]) {
        println!("Yes")
    } else {
        println!("No")
    }
}
