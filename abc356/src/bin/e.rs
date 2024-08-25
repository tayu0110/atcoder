use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut a: [u64; n]}
    a.sort_unstable();
    let t = a.iter().cloned().dedup_with_count().collect::<Vec<_>>();
    let mut cum = vec![0; t.len() + 1];
    for i in 0..t.len() {
        cum[i + 1] = cum[i] + t[i].0;
    }

    // eprintln!("t: {t:?}");
    // eprintln!("cum: {cum:?}");
    let mut res = 0;
    for i in 0..t.len() {
        let (cnt, a) = t[i];
        // eprintln!("i: {i}, cnt: {cnt}, a: {a}, res: {res}");
        res += cnt * cnt.saturating_sub(1) / 2;
        // eprintln!("res: {res}, cnt: {cnt}");

        let mut next = i + 1;
        while next < t.len() {
            let q = t[next].1 / a;
            let pos = t[..].partition_point(|v| v.1 / a <= q);
            // eprintln!("q: {q}, next: {next}, pos: {pos}");
            res += q as usize * (cum[pos] - cum[next]) * cnt;
            next = pos;
        }

        // eprintln!("res: {res}")
    }

    println!("{res}");
}
