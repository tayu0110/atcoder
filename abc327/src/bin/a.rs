use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes}

    if s.windows(2).any(|v| &v[..] == b"ab" || &v[..] == b"ba") {
        println!("Yes")
    } else {
        println!("No")
    }
}
