use proconio::*;

fn main() {
    input! {_: usize, c1: char, c2: char, s: marker::Chars}
    println!(
        "{}",
        s.into_iter()
            .map(|c| if c == c1 { c1 } else { c2 })
            .collect::<String>()
    )
}
