#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {s: Chars}

    let mut set = "abcdefghijklmnopqrstuvwxyz".chars().collect::<std::collections::BTreeSet<_>>();
    for c in s {
        set.remove(&c);
    }

    if set.is_empty() {
        println!("None");
    } else {
        println!("{}", set.iter().next().unwrap());
    }
}
