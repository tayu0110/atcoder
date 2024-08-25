use proconio::*;
use rustc_hash::FxHashMap;
use string::RollingHash;

fn main() {
    input! {n: usize, mut s: [String; n]}

    let mut res = 0;
    let mut memo = vec![FxHashMap::default(); s.iter().map(|s| s.len()).max().unwrap() + 1];
    for s in s {
        let hash = RollingHash::new(&s);
        let mut sum = 0;
        for i in (1..=s.len()).rev() {
            let h = hash.get(..i);
            let d = memo[i].get(&h).unwrap_or(&0) - sum;
            res += d * i;
            sum += d;
            *memo[i].entry(h).or_insert(0) += 1;
        }
    }

    println!("{res}")
}
