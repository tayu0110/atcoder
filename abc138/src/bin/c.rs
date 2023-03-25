#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, v: [usize; n]};

    let mut nt = std::collections::BinaryHeap::new();
    for w in v {
        nt.push(std::cmp::Reverse(w));
    }

    if let Some(std::cmp::Reverse(res)) = nt.pop() {
        let mut res = res as f64;
        while let Some(std::cmp::Reverse(v)) = nt.pop() {
            res = (res + v as f64) / 2.0;
        }

        println!("{}", res);
    }
}
