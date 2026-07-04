use itertools::Itertools;
use proconio::*;
use rustc_hash::{FxHashMap, FxHashSet};
use string::RollingHash;

fn main() {
    input! {q: usize}

    let mut x = FxHashSet::default();
    let mut y = FxHashMap::default();
    let mut inserted = FxHashSet::default();
    let mut res = vec![];
    for i in 0..q {
        input! {t: u8, s: String}

        if t == 1 {
            let hash = RollingHash::new(&s);
            x.insert(hash.get(..));
            if let Some(set) = y.remove(&hash.get(..)) {
                for j in set {
                    inserted.remove(&j);
                }
            }
        } else {
            let hash = RollingHash::new(&s);
            let mut bad = false;
            for i in 1..=s.len() {
                let h = hash.get(..i);
                if x.contains(&h) {
                    bad = true;
                    break;
                }
            }

            if !bad {
                inserted.insert(i);
                for j in 1..=s.len() {
                    y.entry(hash.get(..j))
                        .or_insert_with(|| FxHashSet::default())
                        .insert(i);
                }
            }
        }

        res.push(inserted.len());
    }

    println!("{}", res.iter().join("\n"))
}
