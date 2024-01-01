use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let res = "0123456789"
        .chars()
        .filter(|c| !s.contains(c))
        .next()
        .unwrap();
    println!("{res}")
}
