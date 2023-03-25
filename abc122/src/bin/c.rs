#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, q: usize, s: Chars, p: [(usize, usize); q]}

    let mut d = vec![0; n];
    for i in 1..n {
        d[i] += d[i-1];
        if s[i-1] == 'A' && s[i] == 'C' {
            d[i] += 1;
        }
    }

    for (l, r) in p {
        println!("{}", d[r-1] - d[l-1]);
    }
}
