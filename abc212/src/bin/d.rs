#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {q: usize}

    let mut nt = std::collections::BinaryHeap::new();
    let mut g = 0i64;
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: i64}
            nt.push(std::cmp::Reverse(x - g));
        } else if t == 2 {
            input! {x: i64}
            g += x;
        } else {
            let std::cmp::Reverse(min) = nt.pop().unwrap();
            println!("{}", min + g);
        }
    }
}
