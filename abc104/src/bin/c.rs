#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {d: usize, g: usize, p: [(usize, usize); d]}

    let mut res = std::usize::MAX;
    for v in (0..d).permutations(d) {
        let mut score = 0;
        let mut r = 0;
        for w in v {
            let (p, c) = p[w];
            let i = (w+1) * 100;

            if score + i * p < g {
                score += i * p + c;
                r += p;
            } else {
                r += (g - score + i - 1) / i;
                score = g;
            }

            if score >= g {
                break;
            }
        }

        res = std::cmp::min(res, r);
    }

    println!("{}", res);
}
