use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    if a.windows(2).all(|v| v[0] < v[1]) {
        println!("1");
        return;
    }

    let (mut l, mut r) = (1, n + 1);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut bad = false;

        let mut f = BTreeMap::new();
        let mut len = 0;
        for i in 0..n {
            if a[i] > len {
                len = a[i];
            } else {
                while let Some((&nf, _)) = f.iter().last() {
                    if nf < a[i] {
                        break;
                    }
                    f.remove(&nf);
                }

                *f.entry(a[i] - 1).or_insert(0) += 1;
                let mut cur = a[i] - 1;
                while let Some(&c) = f.get(&cur) {
                    if c < m {
                        break;
                    }

                    f.remove(&cur);

                    cur = cur.wrapping_sub(1);
                    *f.entry(cur).or_insert(0) += 1;
                    if cur > n {
                        bad = true;
                    }
                }

                if bad {
                    break;
                }
                len = a[i];
            }
        }

        if !bad {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r);
}
