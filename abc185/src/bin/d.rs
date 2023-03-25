#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, mut a: [usize; m]}
    a.insert(0, 0);
    a.push(n+1);
    a.sort();

    let mut min = std::usize::MAX;
    let mut d = vec![];

    for v in a.windows(2) {
        if v[1] - v[0] > 1 {
            d.push(v[1] - v[0] - 1);
            min = std::cmp::min(min, v[1] - v[0] - 1);
        }
    }

    let res = d.into_iter().fold(0, |s, v| s + (v + min - 1) / min);
    println!("{}", res);
}
