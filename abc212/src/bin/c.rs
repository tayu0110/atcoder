#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, a: [i64; n], mut b: [i64; m]}

    b.sort();

    let mut res = std::i64::MAX;
    for v in a {
        let (mut l, mut r) = (0, m);

        while r - l > 1 {
            let m = (r + l) / 2;
            if b[m] >= v {
                r = m;
            } else {
                l = m;
            }
        }

        if r < m {
            res = std::cmp::min(res, (b[r]-v).abs());
        }
        res = std::cmp::min(res, (b[l]-v).abs());
    }

    println!("{}", res);
}
