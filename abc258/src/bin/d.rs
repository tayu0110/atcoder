#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, x: usize, p: [(usize, usize); n]};

    let mut res = 10001112223334445556usize;
    let mut sum = 0usize;
    let mut bmin = res;
    for (i, &(a, b)) in p.iter().enumerate() {
        let mut tmp = 0;
        sum += a + b;
        tmp += sum;
        bmin = std::cmp::min(bmin, b);
        let x = if x < i + 1 { 0 } else {x - i - 1 };
        tmp += bmin * x;
        res = std::cmp::min(res, tmp);
    }

    println!("{}", res);
}
