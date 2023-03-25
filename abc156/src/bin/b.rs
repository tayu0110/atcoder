#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut n: usize, k: usize};

    let mut s = vec![];
    while n > 0 {
        s.push(((n % k) as u8 + b'0') as char);
        n /= k;
    }

    println!("{}", s.len());
}
