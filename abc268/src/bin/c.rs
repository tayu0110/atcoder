use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut p: [usize; n]};

    let p = p.into_iter().enumerate().map(|(i, v)| (v, i)).collect_vec();

    let mut r = vec![0; n];
    for (v, i) in p {
        r[(v + n - i) % n] += 1;
    }

    let mut res = 0;
    for i in 0..n {
        res = std::cmp::max(res, r[i] + r[(i+1) % n] + r[(i+n-1) % n]);
    }

    println!("{}", res);
}
