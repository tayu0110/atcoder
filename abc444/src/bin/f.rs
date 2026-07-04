use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut m: usize, mut a: [usize; n]}
        a.sort_unstable();

        let (mut l, mut r) = (1, *a.iter().max().unwrap() + 10);
        while r - l > 1 {
            let mid = (r + l) / 2;
            let mut map = BTreeMap::<usize, usize>::new();
            for &a in &a {
                *map.entry(a).or_default() += 1;
            }

            let mut nm = m;
            while nm > 0
                && let Some((key, value)) = map.pop_last()
            {
                if key < mid {
                    map.insert(key, value);
                    break;
                }
                let lo = key / 2;
                let hi = key - lo;
                if hi < mid {
                    map.insert(key, value);
                    break;
                }

                if nm >= value {
                    nm -= value;
                    *map.entry(hi).or_default() += value;
                    *map.entry(lo).or_default() += value;
                } else {
                    map.insert(key, value - nm);
                    *map.entry(hi).or_default() += nm;
                    *map.entry(lo).or_default() += nm;
                    nm = 0;
                }
            }
            let mut cnt = 0;
            while let Some((key, value)) = map.pop_last() {
                if key < mid {
                    map.insert(key, value);
                    break;
                }
                if cnt + value > (n + m) / 2 + 1 {
                    let diff = cnt + value - ((n + m) / 2 + 1);
                    map.insert(key, diff);
                    break;
                } else {
                    cnt += value;
                }
            }

            if cnt <= (n + m) / 2 {
                r = mid;
                continue;
            }
            while nm > 0
                && let Some((key, value)) = map.pop_last()
            {
                if key == 1 {
                    continue;
                }
                let lo = key / 2;
                let hi = key - lo;
                if nm >= value {
                    nm -= value;
                    *map.entry(hi).or_default() += value;
                    *map.entry(lo).or_default() += value;
                } else {
                    *map.entry(hi).or_default() += nm;
                    *map.entry(lo).or_default() += nm;
                    nm = 0;
                }
            }

            if nm > 0 {
                r = mid;
            } else {
                l = mid;
            }
        }

        println!("{}", l);
    }
}
