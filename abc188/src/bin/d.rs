use proconio::*;
use std::collections::BTreeMap;

fn main() {
    input! {n: usize, c: i64, p: [(usize, usize, i64); n]}

    let mut map = BTreeMap::new();
    for &(a, b, _) in &p {
        map.insert(a, 0);
        map.insert(b + 1, 0);
    }
    let mut cnt = 0;
    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    let mut t = vec![0; cnt];
    let mut memo = vec![0i64; cnt];
    for &(a, b, c) in &p {
        let (na, nb) = (*map.get(&a).unwrap(), *map.get(&(b + 1)).unwrap());
        memo[na] += c;
        memo[nb] -= c;
        t[na] = a;
        t[nb] = b + 1;
    }

    let mut res = 0;
    for i in 0..cnt - 1 {
        memo[i + 1] += memo[i];

        let cost = c.min(memo[i]);
        res += (t[i + 1] - t[i]) * cost as usize;
    }

    println!("{}", res);
}
