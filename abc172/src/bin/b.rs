#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {s: String, t: String}

    println!("{}", s.chars().zip(t.chars()).filter(|&(cs, ct)| cs != ct).count());
}
