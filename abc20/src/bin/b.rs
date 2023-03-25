#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut a: String, b: String};
    a.push_str(b.as_str());
    println!("{}", a.parse::<usize>().unwrap() * 2);
}
