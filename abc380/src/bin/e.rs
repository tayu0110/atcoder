use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut buf = vec![];
    let mut set = (1..=n).map(|i| (i, i, 1)).collect::<BTreeSet<_>>();
    let mut res = vec![1; n + 1];
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {x: usize, c: usize}
            let &(mut t, col, mut cnt) = set
                .range(..=(x, usize::MAX, usize::MAX))
                .next_back()
                .unwrap();
            set.remove(&(t, col, cnt));
            res[col] -= cnt;
            res[c] += cnt;
            if let Some(&(pt, pcol, pcnt)) =
                set.range(..=(t, col, cnt)).next_back().filter(|p| p.1 == c)
            {
                t = pt;
                cnt += pcnt;
                set.remove(&(pt, pcol, pcnt));
            }
            if let Some(&(nt, ncol, ncnt)) = set.range((t, col, cnt)..).next().filter(|n| n.1 == c)
            {
                cnt += ncnt;
                set.remove(&(nt, ncol, ncnt));
            }
            set.insert((t, c, cnt));
        } else {
            input! {c: usize}
            buf.push(res[c]);
        }
    }
    println!("{}", buf.iter().join("\n"))
}
