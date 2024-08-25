use proconio::*;
use rand::{thread_rng, Rng};
use rustc_hash::{FxHashMap, FxHashSet};
use segtree::{Monoid, SegmentTree};

struct ZobristHash;

impl Monoid for ZobristHash {
    type M = u64;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (*l).wrapping_add(*r)
    }
}

fn main() {
    input! {n: usize, q: usize, a: [usize; n], b: [usize; n]}

    let values = a
        .iter()
        .cloned()
        .chain(b.iter().cloned())
        .collect::<FxHashSet<_>>();
    const HASHES: usize = 3;
    let mut rng = thread_rng();
    let mut maps = vec![FxHashMap::default(); HASHES];
    for i in 0..HASHES {
        for &v in &values {
            maps[i].insert(v, rng.r#gen::<u64>());
        }
    }

    let mut sa = vec![
        SegmentTree::<ZobristHash>::new(n),
        SegmentTree::new(n),
        SegmentTree::new(n),
    ];
    for (i, &a) in a.iter().enumerate() {
        for j in 0..HASHES {
            sa[j].set(i, *maps[j].get(&a).unwrap());
        }
    }

    let mut sb = vec![
        SegmentTree::<ZobristHash>::new(n),
        SegmentTree::new(n),
        SegmentTree::new(n),
    ];
    for (i, &b) in b.iter().enumerate() {
        for j in 0..HASHES {
            sb[j].set(i, *maps[j].get(&b).unwrap());
        }
    }

    for _ in 0..q {
        input! {l: usize, r: usize, s: usize, t: usize}

        if r - l != t - s {
            println!("No");
            continue;
        }

        if (0..HASHES).all(|i| sa[i].fold(l - 1..r) == sb[i].fold(s - 1..t)) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
