use proconio::*;

fn main() {
    input! {s: marker::Bytes}
    println!(
        "{}",
        s.into_iter()
            .map(|c| ((c - b'A' + 23) % 26 + b'A') as char)
            .collect::<String>()
    )
}
