#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut a: [usize; 3]}
    a.sort();

    println!("{}", a[1] + a[2]);
}
