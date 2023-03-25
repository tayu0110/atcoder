#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut s: usize}

    let mut res = s;
    s = s % 100 * 10 + s / 100;
    res += s;
    s = s % 100 * 10 + s / 100;
    res += s;

    println!("{}", res);
}
