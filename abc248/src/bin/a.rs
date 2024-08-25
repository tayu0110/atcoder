use proconio::*;

fn main() {
    input! {s: marker::Chars}
    println!(
        "{}",
        "0123456789"
            .chars().find(|c| !s.contains(c))
            .unwrap()
    )
}
