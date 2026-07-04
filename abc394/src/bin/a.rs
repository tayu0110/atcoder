use proconio::*;

fn main() {
    input! {s: marker::Chars}
    println!(
        "{}",
        s.into_iter().filter(|&s| s == '2').collect::<String>()
    )
}
