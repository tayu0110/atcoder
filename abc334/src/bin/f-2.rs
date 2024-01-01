use ordered_float::OrderedFloat;
use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {n: usize, k: usize, sx: i64, sy: i64, mut p: [(i64, i64); n]}
    p.insert(0, (sx, sy));
    p.push((sx, sy));
    let mut dist = 0f64;
    let mut diff = vec![OrderedFloat::default(); n + 2];
    for (i, v) in p.windows(2).enumerate() {
        let (px, py) = v[0];
        let (x, y) = v[1];
        let d = ((y - py) as f64).hypot((x - px) as f64);
        let s = ((x - sx) as f64).hypot((y - sy) as f64);
        let t = ((px - sx) as f64).hypot((py - sy) as f64);
        diff[i + 1] = OrderedFloat::from(s + t - d);
        dist += d;
    }

    let mut set = BTreeSet::new();
    set.insert((OrderedFloat::from(0.0f64), 0));
    for i in 1..=n {
        while let Some((f, prev)) = set.pop_first() {
            if prev < i.saturating_sub(k) {
                continue;
            }

            set.insert((f + diff[i], i));
            set.insert((f, prev));
            break;
        }
    }

    println!(
        "{}",
        dist + set
            .into_iter()
            .find_map(|(f, p)| (p >= n + 1 - k).then_some(f.0))
            .unwrap()
    );
}
