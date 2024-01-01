use proconio::*;

fn main() {
    input! {_: usize, s: marker::Chars}
    println!(
        "{}",
        s.into_iter()
            .map(|c| 1 + (c != 'o') as usize)
            .sum::<usize>()
    )
}
