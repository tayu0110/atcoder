use proconio::*;
use rustc_hash::FxHashMap;

const MAX: usize = 200005;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut fac = vec![usize::MAX; MAX];
    for i in 2..MAX {
        if fac[i] == usize::MAX {
            for j in (2..).take_while(|j| i * j < MAX) {
                fac[i * j] = i;
            }
        }
    }

    let mut map = FxHashMap::default();
    for mut a in a {
        let mut buf = vec![];
        while fac[a] != usize::MAX {
            buf.push(fac[a]);
            a /= fac[a];
        }
        if a != 1 {
            buf.push(a);
        }

        buf.sort_unstable();
        let mut t = vec![];
        for b in buf {
            if t.is_empty() || t.last().unwrap() != &b {
                t.push(b);
            } else {
                t.pop();
            }
        }
        *map.entry(t).or_insert(0usize) += 1;
    }

    let mut res = 0;
    for (t, v) in map {
        if !t.is_empty() && t[0] == 0 {
            res += v * (n - v);
        }
        res += v * v.saturating_sub(1) / 2;
    }

    println!("{res}")
}
