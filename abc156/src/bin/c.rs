#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, x: [i64; n]};

    let mut res = 0x3f3f3f3f3f3f3f3f;
    for now in 0..110 {
        let mut cost = 0;
        for nx in &x {
            cost += (now - *nx) * (now - *nx);
        }

        res = std::cmp::min(res, cost);
    }

    println!("{}", res);
}
