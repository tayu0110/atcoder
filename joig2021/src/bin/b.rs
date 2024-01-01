use itertools::Itertools;
use proconio::*;

fn main() {
    input! {_: usize, k: usize, mut t: marker::Chars}

    let k = k - 1;
    for c in t[k..].iter_mut() {
        if c.is_ascii_lowercase() {
            *c = c.to_ascii_uppercase();
        } else {
            *c = c.to_ascii_lowercase();
        }
    }

    println!("{}", t.iter().join(""))
}
