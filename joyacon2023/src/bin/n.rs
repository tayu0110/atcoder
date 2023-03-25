use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {s: Chars}

    println!(
        "{}",
        s.into_iter()
            .map(|c| if c == 'v' { 1 } else { 2 })
            .sum::<usize>()
    )
}
