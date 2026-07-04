use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars}

    let mut t = vec!['a'; n];
    for (i, c) in s.into_iter().enumerate() {
        t[i] = if c == 'J' {
            'O'
        } else if c == 'O' {
            'I'
        } else {
            'J'
        };
    }
    println!("{}", t.iter().join(""))
}
