#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: [Chars; 3]}

    println!("{}", (0..3).map(|i| s[i][i]).collect::<String>())
}
