#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, t: [i64; n], m: usize, p: [(usize, i64); m]};

    for (id, x) in p {
        let mut t = t.clone();
        t[id-1] = x;
        let res = t.into_iter().sum::<i64>();
        println!("{}", res);
    }
}
