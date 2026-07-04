use std::collections::VecDeque;

use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, mut a: [u32; n], b: [u32; n]}

    let mut res = FxHashMap::default();
    for &a in &a {
        *res.entry(a).or_insert(0) += 1;
    }

    a[0] = b[1];
    *res.entry(a[0]).or_insert(0) += 1;
    let mut rle = VecDeque::new();
    for i in 1..n {
        a[i] = a[i - 1].max(a[i]);
        *res.entry(a[i]).or_insert(0) += n - 1;
        match rle.back_mut() {
            Some((p, cnt)) if *p == a[i] => *cnt += 1,
            _ => rle.push_back((a[i], 1)),
        }
    }

    for i in 2..n {
        *res.entry(b[i]).or_insert(0) += 1;
        let mut cnt = 0;
        while !rle.is_empty() && rle[0].0 <= b[i] {
            let (p, c) = rle.pop_front().unwrap();
            cnt += c;
            *res.entry(p).or_insert(0) -= c * (n - i);
        }
        *res.entry(b[i]).or_insert(0) += cnt * (n - i);
        rle.push_front((b[i], cnt));
    }

    let (mut max, mut cnt) = (0, 0);
    for (k, v) in res {
        if cnt == v {
            max = max.max(k);
        } else if cnt < v {
            max = k;
            cnt = v;
        }
    }

    println!("{max} {cnt}");
}
