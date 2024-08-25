use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Chars; n]}

    let len = s.iter().map(|s| s.len()).max().unwrap();
    let mut t = vec![vec!['*'; n]; len];
    for (i, s) in s.into_iter().enumerate() {
        for (j, s) in s.into_iter().enumerate() {
            t[j][n - 1 - i] = s;
        }
    }

    for mut t in t {
        while !t.is_empty() && t.last() == Some(&'*') {
            t.pop();
        }

        println!("{}", t.iter().join(""));
    }
}
