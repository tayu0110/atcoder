#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, m: usize, x: [usize; n], d: [usize; m]}

    let _g = 0;
    let _res = vec![std::usize::MAX; n];
    let mut map = std::collections::HashMap::new();
    for (i, v) in x.iter().enumerate() {
        map.entry(*v).or_insert(vec![]).push(i);
    }

    let mut nt = x.into_iter().enumerate().map(|(i, x)| std::cmp::Reverse((x, i))).collect::<std::collections::BinaryHeap<_>>();

    for _v in d.into_iter() {
        if let Some(std::cmp::Reverse((_now, _j))) = nt.pop() {

        }
    }
}
