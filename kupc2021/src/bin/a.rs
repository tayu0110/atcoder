#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, s: [usize; n], t: usize};

    let mut nt = s.into_iter().map(|v| std::cmp::Reverse(v)).collect::<std::collections::BinaryHeap<std::cmp::Reverse<usize>>>();

    let mut now = 0;
    let mut res = 0usize;
    while let Some(std::cmp::Reverse(v)) = nt.pop() {
        if v > now {
            now += ((v - now) / t + 1) * t;
            nt.push(std::cmp::Reverse(v));
            res += 1;
        }
    }

    println!("{}", res);
}
