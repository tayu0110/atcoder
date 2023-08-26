use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut res = vec![];
    let mut max = vec![0; m + 1];
    for (i, &a) in a.iter().enumerate() {
        max[a] = i;
    }
    let mut max = max
        .into_iter()
        .enumerate()
        .skip(1)
        .map(|(i, a)| Reverse((a, i)))
        .collect::<BinaryHeap<_>>();
    let Reverse((mut r, _)) = max.iter().next().unwrap();
    let mut nt = BinaryHeap::new();
    for i in 0..=r {
        nt.push(Reverse((a[i], i)));
    }

    let mut used = HashSet::new();
    let mut seen = 0;
    for _ in 0..m {
        while let Some(Reverse((b, i))) = nt.pop() {
            if used.contains(&b) {
                continue;
            }
            if i < seen {
                continue;
            }

            res.push(b);
            used.insert(b);
            seen = i + 1;

            let mut nr = r;
            while let Some(Reverse((i, a))) = max.pop() {
                nr = i;
                if !used.contains(&a) {
                    max.push(Reverse((i, a)));
                    break;
                }
            }

            for i in r + 1..=nr {
                nt.push(Reverse((a[i], i)));
            }
            r = nr;
            break;
        }
    }

    println!("{}", res.into_iter().join(" "))
}
