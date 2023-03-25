#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: i64, op: char, b: i64};
    let res = a + match op {
        '+' => b,
        _ => -b
    };
    println!("{}", res);
}
