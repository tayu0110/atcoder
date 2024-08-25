#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {_n: i64, a: i64, b: i64, p: i64, q: i64, r: i64, s: i64}

    for i in p..=q {
        for j in r..=s {
            if (i - a).abs() == (j - b).abs() {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
