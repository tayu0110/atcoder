use std::collections::BTreeMap;

use fenwick_tree::FenwickTree;
use proconio::*;

fn solve(mut p: Vec<(i64, i64)>) -> i64 {
    p.sort_unstable();
    let n = p.len();

    let map = {
        let mut mp = BTreeMap::new();
        for &(_, y) in &p {
            mp.insert(y, 0);
        }
        let mut cnt = 0;
        for (_, v) in mp.iter_mut() {
            *v = cnt;
            cnt += 1;
        }
        mp
    };

    let mut ft = FenwickTree::<i64>::new(n, 0);
    let mut cnt = FenwickTree::<i64>::new(n, 0);

    let mut sum = 0;
    let mut res = 0;
    for (i, (x, y)) in p.into_iter().enumerate() {
        res += (x * i as i64 - sum) / 2;
        let &idx = map.get(&y).unwrap();
        res += (y * cnt.get_sum(0, idx) - ft.get_sum(0, idx)) / 2;
        res += (ft.get_sum(idx, n) - y * cnt.get_sum(idx, n)) / 2;

        ft.add(idx, y);
        cnt.add(idx, 1);
        sum += x;
    }

    res
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let p = p
        .into_iter()
        .map(|(a, b)| (a + b, a - b))
        .collect::<Vec<_>>();

    let mut res = 0;
    for a in 0..2 {
        for b in 0..2 {
            let p = p
                .iter()
                .filter(|(x, y)| x.rem_euclid(2) == a && y.rem_euclid(2) == b)
                .cloned()
                .collect::<Vec<_>>();

            res += solve(p);
        }
    }

    println!("{res}")
}
