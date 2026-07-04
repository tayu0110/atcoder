use std::collections::BTreeMap;

use proconio::*;

fn main() {
    input! {mut n: usize, mut m: usize, k: usize, mut s: [marker::Bytes; n]}

    if n < m {
        let mut ns = vec![vec![0; n]; m];
        for i in 0..n {
            for j in 0..m {
                ns[j][i] = s[i][j];
            }
        }
        s = ns;
        (n, m) = (m, n);
    }

    let mut cum = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            cum[i + 1][j + 1] = (s[i][j] == b'#') as usize;
        }
    }
    for i in 1..=n {
        for j in 0..m {
            cum[i][j + 1] += cum[i][j];
        }
    }
    for i in 0..n {
        for j in 0..=m {
            cum[i + 1][j] += cum[i][j];
        }
    }

    let mut ret = 0usize;
    for l in 0..m {
        for r in l + 1..=m {
            let width = r - l;
            let height = k / width;

            let mut set = BTreeMap::new();
            for i in 0..=n {
                let x = cum[i][r] - cum[i][l];
                let entry = set.entry(x).or_insert(0);
                ret += *entry;
                *entry += 1;

                if i >= height {
                    let y = cum[i - height][r] - cum[i - height][l];
                    *set.entry(y).or_insert(0) -= 1;
                }
            }
        }
    }

    println!("{ret}")
}
