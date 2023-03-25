#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, a: usize, b: usize, mut d: [usize; b]};
    d.push(0);
    d.push(n+1);
    d.sort();
    let mut res = n - b;
    for i in 0..b+1 {
        res -= (d[i+1] - d[i] - 1) / a;
    }
    println!("{}", res);
}
