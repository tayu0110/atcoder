#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut n: usize};

    let mut res = 0;
    res += n / 11 * 2;
    n %= 11;
    res += n / 6;
    n %= 6;
    if n > 0 {
        res += 1;
    }
    println!("{}", res);
}
