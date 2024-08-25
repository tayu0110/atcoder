use proconio::*;
use std::collections::BTreeMap;

fn mset_insert(set: &mut BTreeMap<usize, usize>, val: usize) {
    *set.entry(val).or_insert(0) += 1;
}

fn mset_remove(set: &mut BTreeMap<usize, usize>, val: usize) {
    *set.entry(val).or_insert(1) -= 1;
    if *set.get(&val).unwrap() == 0 {
        set.remove(&val);
    }
}

fn main() {
    input! {q: usize}

    let mut min = BTreeMap::new();

    let mut multiset = BTreeMap::new();
    let mut dup = 0;

    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize}
            if multiset.contains_key(&x) {
                if *multiset.get(&x).unwrap() == 1 {
                    dup += 1;
                }
                *multiset.entry(x).or_insert(0) += 1;
            } else {
                if let (Some((k, _)), Some((k2, _))) =
                    (multiset.range(x..).next(), multiset.range(..=x).next_back())
                {
                    mset_insert(&mut min, k ^ x);
                    mset_insert(&mut min, k2 ^ x);
                    mset_remove(&mut min, k ^ k2);
                } else if let Some((k, _)) = multiset.range(x..).next() {
                    mset_insert(&mut min, k ^ x);
                } else if let Some((k, _)) = multiset.range(..=x).next_back() {
                    mset_insert(&mut min, k ^ x);
                }

                multiset.insert(x, 1);
            }
        } else if t == 2 {
            input! {x: usize}
            *multiset.entry(x).or_insert(1) -= 1;
            if *multiset.get(&x).unwrap() == 0 {
                multiset.remove(&x);
                if let (Some((k, _)), Some((k2, _))) =
                    (multiset.range(x..).next(), multiset.range(..=x).next_back())
                {
                    mset_remove(&mut min, k ^ x);
                    mset_remove(&mut min, k2 ^ x);
                    mset_insert(&mut min, k ^ k2);
                } else if let Some((k, _)) = multiset.range(x..).next() {
                    mset_remove(&mut min, k ^ x);
                } else if let Some((k, _)) = multiset.range(..=x).next_back() {
                    mset_remove(&mut min, k ^ x);
                }
            } else if *multiset.get(&x).unwrap() == 1 {
                dup -= 1;
            }
        } else if dup > 0 {
            println!("0");
        } else {
            println!("{}", min.iter().next().unwrap().0);
        }
    }
}
