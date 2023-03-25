#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: [usize; 4]}

    println!("{}", a.iter().min().unwrap());
}
