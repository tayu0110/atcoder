#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {a: usize, b: usize, c: usize};

    let mut t = [a, b, c];
    t.sort();

    if let [a, b, c] = t[0..3] {
        if a + b < c {
            println!("-1");
        } else {
            println!("{}", c);
        }
    }
}
