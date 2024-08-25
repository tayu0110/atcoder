#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize, e: usize};

    let mut v = [a, b, c, d, e];
    v.sort();

    if v[0] == v[1] && v[3] == v[4] {
        if v[0] == v[2] && v[2] != v[3] {
            println!("Yes");
        } else if v[0] != v[2] && v[2] == v[3] {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
