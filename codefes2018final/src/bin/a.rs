use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, m: usize}

    let mut res = 0usize;
    let mut t = vec![FxHashMap::default(); n];
    for _ in 0..m {
        input! {a: usize, b: usize, l: u16}
        res += t[a - 1].get(&(2540 - l)).unwrap_or(&0);
        res += t[b - 1].get(&(2540 - l)).unwrap_or(&0);
        *t[a - 1].entry(l).or_insert(0) += 1;
        *t[b - 1].entry(l).or_insert(0) += 1;
    }

    println!("{res}");
}
