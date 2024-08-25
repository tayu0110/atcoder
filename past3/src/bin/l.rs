use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {n: usize}

    let mut idx = vec![0; 300001];
    let mut cnt = 0;
    let mut s = vec![];
    let (mut w0, mut w1) = (BTreeSet::new(), BTreeSet::new());
    for i in 0..n {
        input! {k: usize, t: [usize; k]}

        s.push(
            t.into_iter()
                .enumerate()
                .map(|(i, t)| (t, i + cnt))
                .rev()
                .collect::<Vec<_>>(),
        );

        w0.insert(s[i].pop().unwrap());
        if !s[i].is_empty() {
            let q = s[i].pop().unwrap();
            w0.insert(q);
            w1.insert(q);
        }

        for _ in 0..k {
            idx[cnt] = i;
            cnt += 1;
        }
    }

    input! {m: usize, a: [usize; m]}

    for a in a {}
}
