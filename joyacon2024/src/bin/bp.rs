use itertools::Itertools;
use proconio::*;

fn main() {
    input! {mut s: marker::Chars, t: [usize; 4]}

    for t in t.into_iter().rev() {
        if t == s.len() {
            s.push('"');
        } else {
            s.insert(t, '"');
        }
    }

    println!("{}", s.iter().join(""))
}
