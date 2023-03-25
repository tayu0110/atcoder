#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {s: usize, mut t: usize, x: usize}

    if t < s {
        t += 24;
    }

    if s <= x && x < t {
        println!("Yes");
    } else if s <= x + 24 && x + 24 < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
