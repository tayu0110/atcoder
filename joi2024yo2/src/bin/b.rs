use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {n: usize, _: usize, q: usize, p: [(usize, usize); n]}

    let mut cum = vec![0; n + 1];
    let mut tree = vec![HashMap::new(); n << 1];
    for (i, (p, a)) in p.into_iter().enumerate() {
        tree[i + n].insert(a, p);
        cum[i + 1] = cum[i] + p;
    }
    for i in (1..n).rev() {
        let (pt, nt) = tree.split_at_mut(i << 1);
        pt[i] = nt[0].clone();
        for (&k, &v) in &nt[1] {
            *pt[i].entry(k).or_insert(0) += v;
        }
    }

    for _ in 0..q {
        input! {t: usize, l: usize, r: usize}

        let sum = {
            let (mut l, mut r) = (l + n - 1, r + n);
            let mut sum = 0;
            while l < r {
                if l & 1 != 0 {
                    sum += tree[l].get(&t).unwrap_or(&0);
                    l += 1;
                }
                if r & 1 != 0 {
                    sum += tree[r - 1].get(&t).unwrap_or(&0);
                }
                l >>= 1;
                r >>= 1;
            }
            sum
        };

        println!("{}", cum[r] - cum[l - 1] - sum / 2);
    }
}
