#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize};
    let mut now = 1usize;
    let mut res = 0usize;
    for _ in 0..=n {
        res += now;
        now *= 5;
    }
    println!("{}", res);
}
