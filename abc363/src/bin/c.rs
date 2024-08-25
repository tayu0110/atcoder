use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, k: usize, s: marker::Chars}

    let mut checked = FxHashSet::default();
    let mut res = 0;
    for s in s.iter().permutations(n) {
        let t = s.iter().cloned().collect::<String>();
        if checked.contains(&t) {
            continue;
        }
        checked.insert(t);
        if s.windows(k)
            .all(|s| s.iter().zip(s.iter().rev()).any(|(a, b)| a != b))
        {
            res += 1;
        }
    }

    println!("{res}")
}
