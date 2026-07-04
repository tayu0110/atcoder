use proconio::*;
use rustc_hash::FxHashMap;

fn solve2(a: &[i32], b: &[i32]) -> usize {
    let n = a.len() - 1;
    let mut map = FxHashMap::default();
    map.insert(0, 1);
    let mut ret = 0;
    for i in 1..n + 1 {
        let diff = a[i] - b[i];
        ret += *map.get(&diff).unwrap_or(&0);
        *map.entry(diff).or_insert(0) += 1;
    }
    ret
}

fn main() {
    input! {n: usize, s: marker::Bytes}

    let mut a = vec![0i32; n + 1];
    let mut b = vec![0i32; n + 1];
    let mut c = vec![0i32; n + 1];
    let mut p = vec![(0, 0, 0)];
    for (i, ch) in s.into_iter().enumerate() {
        a[i + 1] += a[i];
        b[i + 1] += b[i];
        c[i + 1] += c[i];
        if ch == b'A' {
            a[i + 1] += 1;
        } else if ch == b'B' {
            b[i + 1] += 1;
        } else {
            c[i + 1] += 1;
        }
        p.push((
            a[i + 1] - b[i + 1],
            a[i + 1] - c[i + 1],
            b[i + 1] - c[i + 1],
        ));
    }

    let mut ret = 0usize;

    ret += solve2(&a, &b);
    ret += solve2(&a, &c);
    ret += solve2(&b, &c);
    let mut map = FxHashMap::default();
    for p in p {
        ret -= *map.get(&p).unwrap_or(&0) * 2;
        *map.entry(p).or_insert(0) += 1;
    }

    println!("{}", n * (n + 1) / 2 - ret);
}
