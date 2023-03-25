#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {_n: usize, s: Chars};

    let s = s.into_iter().fold(vec![], |mut v, c| {
        match v.last() {
            Some(l) if *l == c => { v },
            _ => { v.push(c); v }
        }
    });

    println!("{}", s.len());
}
