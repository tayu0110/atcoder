use proconio::*;
use segtree::{Monoid, SegmentTree};

struct F64Min;

impl Monoid for F64Min {
    type M = f64;
    fn id() -> Self::M {
        f64::MAX
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        if l < r {
            *l
        } else {
            *r
        }
    }
}

fn main() {
    input! {n: usize, k: usize, sx: i64, sy: i64, mut p: [(i64, i64); n]}
    p.insert(0, (sx, sy));

    let mut dist = vec![0.0f64; n + 1];
    let mut diff = vec![0.0f64; n + 1];
    for (i, v) in p.windows(2).enumerate() {
        let (px, py) = v[0];
        let (x, y) = v[1];
        let d = ((y - py) as f64).hypot((x - px) as f64);
        dist[i + 1] = dist[i] + d;

        let s = ((sy - py) as f64).hypot((sx - px) as f64);
        let t = ((sy - y) as f64).hypot((sx - x) as f64);
        diff[i + 1] = (s + t) - d;
    }

    // eprintln!("diff: {diff:?}");

    let mut st = SegmentTree::<F64Min>::new(n + 1);
    st.set(0, 0.0);
    for i in 1..=n {
        let d = st.fold(i.saturating_sub(k)..i) + diff[i];
        st.set(i, d);
    }

    if cfg!(debug_assertions) {
        // let st = (0..n + 1).map(|i| st.fold(i..i + 1)).collect::<Vec<_>>();
        // eprintln!("st: {st:?}");
    }

    let (x, y) = p[n];
    println!(
        "{}",
        st.fold(n + 1 - k..n + 1) + dist[n] + ((x - sx) as f64).hypot((y - sy) as f64)
    )
}
