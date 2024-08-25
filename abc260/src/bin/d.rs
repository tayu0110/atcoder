use std::collections::BTreeMap;

#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, k: usize, a: [usize; n]};

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut res = vec![-1; n+1];
    let mut list = vec![0; n+1];
    for (i, w) in a.into_iter().enumerate() {
        if let Some((key, v)) = map.range((std::ops::Bound::Excluded(w), std::ops::Bound::Unbounded)).next() {
            let (key, v) = (*key, *v);
            let nv = v + 1;
            map.remove(&key);
            list[w] = key;
            if nv == k {
                let mut now = w;
                while now != 0 {
                    res[now] = i as i32 + 1;
                    now = list[now];
                }
            } else {
                map.insert(w, nv);
            }
        } else if k == 1 {
            res[w] = i as i32 + 1;
        } else {
            map.insert(w, 1usize);
        }
    }

    for i in 1..=n {
        println!("{}", res[i]);
    }
}
