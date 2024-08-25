#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {l: usize, r: usize};
    let v = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    for i in l-1..r {
        print!("{}", v[i]);
    }
    println!();
}
