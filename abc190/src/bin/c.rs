#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, a: [(usize, usize); m], k: usize, c: [(usize, usize); k]}

    let mut res = 0;
    for i in 0..1<<k {
        let mut d = vec![0; n+1];
        for j in 0..k {
            if i & (1 << j) == 0 {
                d[c[j].0] += 1;
            } else {
                d[c[j].1] += 1;
            }
        }

        let mut r = 0;
        for (a, b) in &a {
            if d[*a] > 0 && d[*b] > 0 {
                r += 1;
            }
        }

        res = std::cmp::max(res, r);
    }

    println!("{}", res);
}
