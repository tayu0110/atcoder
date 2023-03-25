#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}

    let mut map_x = std::collections::BTreeMap::new();
    let mut map_y = std::collections::BTreeMap::new();
    for &(x, y) in &p {
        map_x.insert(x, 1);
        map_y.insert(y, 1);
    }

    let cnt_x = {
        let mut cnt = 0;
        for (_, v) in map_x.iter_mut() {
            *v = cnt;
            cnt += 1;
        }
        cnt
    };
    let cnt_y = {
        let mut cnt = 0;
        for (_, v) in map_y.iter_mut() {
            *v = cnt;
            cnt += 1;
        }
        cnt
    };

    let mut xs = vec![vec![]; cnt_x];
    let mut ys = vec![vec![]; cnt_y];
    for (x, y) in p.iter_mut() {
        *x = *map_x.get(x).unwrap();
        *y = *map_y.get(y).unwrap();
        xs[*x].push(*y);
        ys[*y].push(*x);
    }
}
