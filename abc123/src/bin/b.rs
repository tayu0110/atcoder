use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: [usize; 5]}

    let mut res = 100000000;
    for v in (0..5).permutations(5) {
        let mut sum = 0;
        for i in 0..4 {
            sum += (a[v[i]] + 9) / 10 * 10;
        }
        sum += a[v[4]];
        res = std::cmp::min(res, sum);
    }

    println!("{}", res);
}
