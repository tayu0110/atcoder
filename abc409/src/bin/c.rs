use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, l: usize, mut d: [usize; n - 1]}

    if l % 3 != 0 {
        println!("0");
        return;
    }
    d.insert(0, 0);
    for i in 0..n - 1 {
        d[i + 1] += d[i];
        d[i + 1] %= l;
    }
    let mut memo = FxHashMap::default();
    for &d in &d {
        *memo.entry(d).or_insert(0usize) += 1;
    }

    let mut cnt = 0;
    let mut pair = vec![];
    for &d in memo.keys() {
        let e = (d + l / 3) % l;
        let f = (d + l - l / 3) % l;
        let mut t = [d, e, f];
        t.sort();
        pair.push(t);
    }
    pair.sort_unstable();
    pair.dedup();

    for [d, e, f] in pair {
        cnt += memo.get(&d).unwrap_or(&0) * memo.get(&e).unwrap_or(&0) * memo.get(&f).unwrap_or(&0);
    }

    println!("{cnt}")
}
