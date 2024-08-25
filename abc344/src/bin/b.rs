use std::io::Read;

use itertools::Itertools;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).ok();
    println!("{}", s.trim_end().split('\n').rev().join("\n"));
}
