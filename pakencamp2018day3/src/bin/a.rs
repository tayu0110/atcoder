#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {y: usize, m: usize, d: usize};
    if m != 12 || d != 25 {
        println!("NOT CHRISTMAS DAY");
    } else {
        println!("{}", y - 2018);
    }
}
