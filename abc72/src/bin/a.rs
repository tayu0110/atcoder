#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: usize, t: usize}

    println!("{}", x.saturating_sub(t));
}
