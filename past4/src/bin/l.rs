use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, q: usize, h: [i64; n]}

    let mut diff = h.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
    let mut odd = FxHashMap::default();
    let mut even = FxHashMap::default();
    for (i, &d) in diff.iter().enumerate() {
        if i % 2 == 0 {
            *odd.entry(d).or_insert(0) += 1;
        } else {
            *even.entry(d).or_insert(0) += 1;
        }
    }

    let mut o_offset = 0;
    let mut e_offset = 0;
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {v: i64}
            o_offset += v;
            e_offset -= v;
        } else if ty == 2 {
            input! {v: i64}
            e_offset += v;
            o_offset -= v;
        } else {
            input! {mut u: usize, v: i64}
            u -= 1;

            let (l, r) = if u % 2 == 0 {
                (&mut odd, &mut even)
            } else {
                (&mut even, &mut odd)
            };
            if u < n - 1 {
                *l.entry(diff[u]).or_insert(0) -= 1;
                diff[u] -= v;
                *l.entry(diff[u]).or_insert(0) += 1;
            }
            if u > 0 {
                *r.entry(diff[u - 1]).or_insert(0) -= 1;
                diff[u - 1] += v;
                *r.entry(diff[u - 1]).or_insert(0) += 1;
            }
        }

        println!(
            "{}",
            odd.get(&o_offset).unwrap_or(&0) + even.get(&e_offset).unwrap_or(&0)
        )
    }
}
