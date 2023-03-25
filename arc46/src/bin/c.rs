use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, mut p: [(usize, usize); n], q: [(usize, usize); m]};
    let mut q = q.into_iter().map(|(l, r)| (r, l)).collect_vec();
    q.sort();
    p.sort();

    let mut qi = 0;
    let mut res = 0;
    let mut map = std::collections::BTreeMap::new();
    for (a, b) in p {
        while qi < m && q[qi].0 <= a {
            let (_, c) = q[qi];
            *map.entry(c).or_insert(0) += 1;
            qi += 1;
        }

        if let Some((k, v)) = map.range_mut(b..).next() {
            res += 1;
            *v -= 1;
            let k = *k;
            if *v == 0 {
                map.remove(&k);
            }
        }
    }

    println!("{}", res);
}
