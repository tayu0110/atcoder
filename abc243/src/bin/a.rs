#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut v: usize, a: usize, b: usize, c: usize}

    v %= a + b + c;
    if a > v {
        println!("F");
    } else {
        v -= a;
        if b > v {
            println!("M");
        } else {
            println!("T");
        }
    }
}
