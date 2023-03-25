#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {_n: usize, x: i32, s: Chars}

    println!("{}", s.into_iter().fold(x, |x, c| if c == 'o' { x + 1 } else { std::cmp::max(0, x - 1) }));
}
