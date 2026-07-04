use proconio::*;
use rustc_hash::FxHashMap;
use string::RollingHash;

fn main() {
    input! {n: usize, s: [String; n]}

    let len = s.iter().map(|s| s.len()).max().unwrap();
    let mut memo = vec![FxHashMap::default(); len + 1];
    for s in s {
        let mut res = s.len();
        let rh = RollingHash::new(&s);
        for i in 1..=s.len() {
            let hash = rh.get(..i).raw_value();
            res = res.min(*memo[i].get(&hash).unwrap_or(&(usize::MAX >> 3)) - i + s.len() - i);
            let entry = memo[i].entry(hash).or_insert(usize::MAX);
            *entry = s.len().min(*entry);
        }
        println!("{res}");
    }
}
