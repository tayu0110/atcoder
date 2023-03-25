#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {s: Chars}

    for (i, v) in s.windows(2).enumerate() {
        if v[0] == v[1] {
            println!("{} {}", i+1, i+2);
            return;
        }
    }

    for (i, v) in s.windows(3).enumerate() {
        if v[0] == v[2] {
            println!("{} {}", i+1, i+3);
            return;
        }
    }

    println!("-1 -1");
}
