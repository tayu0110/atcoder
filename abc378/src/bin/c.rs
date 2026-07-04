use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, a: [usize; n]}
    let mut memo = FxHashMap::default();
    println!(
        "{}",
        a.into_iter()
            .enumerate()
            .map(|(i, a)| memo.insert(a, i as i32 + 1).unwrap_or(-1))
            .join(" ")
    );
}
