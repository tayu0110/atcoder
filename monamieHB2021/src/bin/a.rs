#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut n: usize};
    n /= 2;
    n %= 13;

    match n {
        0 => println!("0"),
        4 | 5 => println!("1"),
        8..=10 => println!("2"),
        1 | 2 | 12 => println!("3"),
        3 | 6 | 7 => println!("4"),
        11 => println!("5"),
        _ => unreachable!()
    }
}
