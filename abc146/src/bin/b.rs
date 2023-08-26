use proconio::*;

fn main() {
    input! {n: u8, s: marker::Bytes}
    println!(
        "{}",
        s.into_iter()
            .map(|c| ((c - b'A' + n) % 26 + b'A') as char)
            .collect::<String>()
    )
}
