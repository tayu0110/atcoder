use std::collections::BTreeSet;

use proconio::*;

fn main() {
    input! {k: usize, n: usize, e: [(usize, usize); n]}

    let mut set = e.into_iter().collect::<BTreeSet<_>>();
    let mut t = vec![];
    'b: while let Some((p, q)) = set.pop_last() {
        let (mut a, mut b) = (p, q);
        while a > 0 {
            let (na, nb) = (a - 1, (b + 1) / 2);
            if set.contains(&(na, nb)) {
                continue 'b;
            }
            (a, b) = (na, nb);
        }

        t.push((p, q));
    }

    let mut res = (2u64 << k) - 1;
    for (p, _) in t {
        res -= (2 << (k - p)) - 1;
    }

    println!("{res}")
}
