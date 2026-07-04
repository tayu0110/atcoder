use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut memo = FxHashMap::default();
    for a in a {
        if let Some(&prev) = memo.get(&(a - 1)) {
            let entry = memo.entry(a).or_insert(1);
            *entry = (prev + 1).max(*entry);
        } else {
            memo.entry(a).or_insert(1);
        }
    }

    println!("{}", memo.values().max().unwrap())
}
