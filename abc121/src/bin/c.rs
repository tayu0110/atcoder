#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut m: usize, mut p: [(usize, usize); n]};

    p.sort();
    let mut res = 0;
    for (a, b) in p {
        if b > m {
            res += a * m;
            m = 0;
        } else {
            res += a * b;
            m -= b;
        }

        if m == 0 {
            break;
        }
    }

    println!("{}", res);
}
