use proconio::*;

fn main() {
    input! {s: marker::Bytes}
    println!(
        "{}",
        s.into_iter()
            .filter(|s| s.is_ascii_digit())
            .map(|c| c as char)
            .collect::<String>()
    )
}
