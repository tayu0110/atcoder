use proconio::*;

fn main() {
    input! {s: marker::Chars}
    let t = s
        .into_iter()
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>();
    println!("{}", t)
}
