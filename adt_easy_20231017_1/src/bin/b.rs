use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let res = "0123456789"
        .chars().find(|c| !s.contains(c))
        .unwrap();
    println!("{res}")
}
