#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, d: usize, mut p: [(usize, usize); n]}

    p.sort_by_key(|(_, r)| *r);

    let mut destroyed = 0;

    let mut res = 0;
    for (l, r) in p {
        if l > destroyed {
            res += 1;
            destroyed = r + d - 1;
        }
    }

    println!("{}", res);
}
