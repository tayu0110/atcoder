use fenwick_tree::FenwickTree;
use proconio::*;
use std::collections::BTreeMap;

fn main() {
    input! {n: usize, m: usize, mut a: [[u32; m]; n]}
    let cnt = {
        let mut map = BTreeMap::new();
        for i in 0..n {
            for j in 0..m {
                map.insert(a[i][j], 0);
            }
        }
        let mut cnt = 1;
        for (_, v) in map.iter_mut() {
            *v = cnt;
            cnt += 1;
        }
        for i in 0..n {
            for j in 0..m {
                let new = *map.get(&a[i][j]).unwrap();
                a[i][j] = new;
            }
        }
        cnt as usize
    };

    let mut ft = FenwickTree::new(cnt, 0usize);
    let v = a.pop().unwrap();
    for v in v {
        ft.add(v as usize, 1);
    }

    let mut res = 0;
    for (i, v) in a.iter().rev().enumerate() {
        for (k, &v) in v.iter().enumerate() {
            res += (k + 1) * (i + 1) + ft.get_sum(0, v as usize);
        }
        for &v in v {
            ft.add(v as usize, 1);
        }
    }

    println!("{}", res)
}
