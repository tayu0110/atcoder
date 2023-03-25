#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {_h: usize, _w: usize, n: usize, p: [(usize, usize); n]}

    let mut map_a = std::collections::BTreeMap::new();
    let mut map_b = std::collections::BTreeMap::new();
    for (a, b) in p.iter().cloned() {
        map_a.insert(a, 0);
        map_b.insert(b, 0);
    }
    let mut cnt = 1;
    for (_, v) in map_a.iter_mut() {
        *v = cnt;
        cnt += 1;
    }
    cnt = 1;
    for (_, v) in map_b.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    for (a, b) in p {
        println!("{} {}", map_a.get(&a).unwrap(), map_b.get(&b).unwrap());
    }
}
