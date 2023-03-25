#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut p: [usize; n]}
    p.sort();

    let mut res = p.iter().take(n-1).sum::<usize>();
    res += *p.last().unwrap() / 2;

    println!("{}", res);
}
