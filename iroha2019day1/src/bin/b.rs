#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut s: Chars, k: usize};
    let n = s.len();
    s.rotate_left(k % n);
    let s = s.into_iter().fold(String::new(), |mut s, v| {s.push(v); s});
    println!("{}", s);
}
