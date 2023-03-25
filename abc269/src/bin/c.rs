#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut x: usize};

    let base = x;
    let mut res = vec![];
    while x > 0 {
        res.push(x);
        x = (x - 1) & base;
    }
    res.push(0);
    res.sort();

    for v in res {
        println!("{}", v);
    }
}
