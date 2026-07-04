use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut res = 0usize;
    let mut map = FxHashMap::default();
    for (i, (mut a, mut b)) in p.into_iter().enumerate() {
        if a > b {
            (a, b) = (b, a);
        }
        a -= 1;
        b -= 1;

        if a < n - b {
            let d = a;
            a -= d;
            b += d;
        } else if a == n - b {
            let d = a - 1;
            a -= d;
            b += d;
        } else {
            let d = n - b;
            a -= d;
            b += d;
        }
        a %= n;
        b %= n;
        if a > b {
            (a, b) = (b, a);
        }

        res += i;
        if let Some(r) = map.get(&(a, b)) {
            res -= r;
        }
        *map.entry((a, b)).or_insert(0) += 1;
    }

    println!("{res}")
}
