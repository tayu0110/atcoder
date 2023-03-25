use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut a: [usize; n]};

    a.sort();
    a.reverse();

    let mut res = 0usize;
    for v in (0..3).permutations(3) {
        let mut s = String::new();
        for w in v {
            s.push_str(a[w].to_string().as_str());
        }

        res = std::cmp::max(res, s.parse().unwrap());
    }

    println!("{}", res);
}
