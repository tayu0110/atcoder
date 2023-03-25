#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {mut a: [usize; 3]}
    a.sort();

    if a[0] == a[1] {
        println!("{}", a[2]);
    } else if a[1] == a[2] {
        println!("{}", a[0]);
    } else {
        println!("0");
    }
}
