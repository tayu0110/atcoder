use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    let mut map = BTreeMap::new();
    for &a in &a {
        map.insert(a, 0);
    }
    let mut cnt = 1;
    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    a.iter_mut().for_each(|a| *a = *map.get(a).unwrap());
    println!("{}", a.iter().join(" "));
}
