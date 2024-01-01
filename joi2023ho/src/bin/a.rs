use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut last = HashMap::new();
    let mut v: Vec<(usize, usize)> = vec![];
    for a in a {
        if last.contains_key(&a) {
            let mut cnt = 1;
            while let Some((na, c)) = v.pop() {
                cnt += c;
                let t = last.get_mut(&na).unwrap();
                *t -= c;
                if *t == 0 {
                    drop(t);
                    last.remove(&na);
                }
                if na == a {
                    break;
                }
            }
            *last.entry(a).or_insert(0) += cnt;
            v.push((a, cnt));
        } else {
            v.push((a, 1));
            *last.entry(a).or_insert(0) += 1;
        }
    }

    println!(
        "{}",
        v.into_iter()
            .map(|(a, cnt)| vec![a; cnt])
            .flatten()
            .join("\n")
    )
}
