#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {_n: usize, k: usize, mut s: Chars};

    s[k-1] = s[k-1].to_lowercase().next().unwrap();

    let s = s.into_iter().fold(String::new(), |mut s, c| {
        s.push(c);
        s
    });

    println!("{}", s);
}
