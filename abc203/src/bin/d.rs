#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, k: usize, a: [[i64; n]; n]}

    let (mut l, mut r) = (-1i64, 1000000100i64);
    while r - l > 1 {
        let m = (r + l) / 2;

        let mut na = vec![vec![0; n+1]; n+1];
        for i in 0..n {
            for j in 0..n {
                if a[i][j] > m {
                    na[i+1][j+1] = 1;
                } else {
                    na[i+1][j+1] = 0;
                }
            }
        }

        for i in 0..=n {
            for j in 1..=n {
                na[i][j] += na[i][j-1];
            }
        }
        for i in 1..=n {
            for j in 0..=n {
                na[i][j] += na[i-1][j];
            }
        }

        // eprintln!("na: {:?}", na);

        let mut bad = true;
        for i in k..=n {
            for j in k..=n {
                let t = na[i][j] + na[i-k][j-k] - na[i-k][j] - na[i][j-k];
                // eprintln!("m: {}, t: {}, i: {}, j: {}", m, t, i, j);

                if t <= k*k/2 {
                    bad = false;
                }
            }
        }

        if bad {
            l = m;
        } else {
            r = m;
        }
    }

    // eprintln!("l: {}, r: {}", l, r);
    println!("{}", r);
}
