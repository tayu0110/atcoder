#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, m: usize, x: [usize; n], d: [usize; m]}

    let mut g = 0;
    let mut res = vec![std::usize::MAX; n];
    let mut map = std::collections::HashMap::new();
    for (i, v) in x.iter().enumerate() {
        map.entry(*v).or_insert(vec![]).push(i);
    }

    let mut nt = x.into_iter().enumerate().map(|(i, x)| std::cmp::Reverse((x, i))).collect::<std::collections::BinaryHeap<_>>();

    for (i, v) in d.into_iter().enumerate() {
        if let Some(std::cmp::Reverse((now, j))) = nt.pop() {

        }
    }
}
