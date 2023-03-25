#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, h: [usize; n]}

    let mut map = std::collections::BTreeMap::new();
    for v in &h {
        map.insert(*v, 1);
    }
    let mut cnt = 1;
    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    for v in h {
        println!("{}", map.get(&v).unwrap());
    }
}
