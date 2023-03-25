#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, a: [usize; n]};

    let mut nt = a.into_iter().collect::<std::collections::BinaryHeap<_>>();

    for _ in 0..m {
        let na = nt.pop().unwrap();
        nt.push(na / 2);
    }

    let res = nt.into_iter().sum::<usize>();

    println!("{}", res);
}
