use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, k: usize, t: [[usize; n]; n]};

    let mut res = 0;
    for mut v in (1..n).permutations(n-1) {
        v.insert(0, 0);
        v.push(0);
        let mut time = 0;
        for v in v.windows(2) {
            time += t[v[1]][v[0]];
        }

        if time == k {
            res += 1;
        }
    }

    println!("{}", res);
}
