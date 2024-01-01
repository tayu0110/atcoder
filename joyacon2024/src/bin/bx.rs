use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut s: marker::Chars}
    s[0] = s[0].to_ascii_uppercase();
    s[1..].iter_mut().for_each(|c| *c = c.to_ascii_lowercase());

    println!("{}", s.iter().join(""))
}
