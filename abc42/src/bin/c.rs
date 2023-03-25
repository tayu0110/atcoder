#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

// #[fastout]
fn main() {
    input! {n: usize, k: usize, d: [char; k]}

    for i in n..=10*n {
        let s = i.to_string();
        if s.chars().any(|c| d.iter().any(|d| c == *d)) {
            continue;
        }

        println!("{}", i);
        std::process::exit(0);
    }
}
