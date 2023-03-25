#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, s: usize, d: usize, p: [(usize, usize); n]}

    for (x, y) in p {
        if x < s && y > d {
            println!("Yes");
            std::process::exit(0);
        }
    }

    println!("No");
}
