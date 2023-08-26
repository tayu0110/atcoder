use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let map = {
        let mut map = BTreeMap::new();
        for &a in &a {
            map.insert(a, 0);
        }
        let mut cnt = 0;
        for (_, v) in map.iter_mut() {
            *v = cnt;
            cnt += 1;
        }
        map
    };

    for a in a {
        let idx = *map.get(&a).unwrap();
        println!("{}", k / n + (idx < k % n) as usize)
    }
}
