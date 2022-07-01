#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, x: usize};

    let mut x = x - 1;
    x /= n;

    let x = x;

    println!("{}", "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string().chars().collect::<Vec<char>>()[x]);
}
