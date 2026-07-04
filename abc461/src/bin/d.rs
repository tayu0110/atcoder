use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {h: usize, w: usize, k: usize, s: [marker::Bytes; h]}

    let mut cum = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            cum[i + 1][j + 1] = (s[i][j] == b'1') as usize;
        }
    }
    for i in 0..h {
        for j in 0..w {
            cum[i + 1][j + 1] += cum[i + 1][j];
        }
    }
    for j in 0..w {
        for i in 0..h {
            cum[i + 1][j + 1] += cum[i][j + 1];
        }
    }
    let mut ret = 0usize;
    for c in 1..=w {
        for j in 0..w {
            if j + c > w {
                break;
            }
            let mut map = FxHashMap::default();
            for i in 0..=h {
                let t = cum[i][j + c] - cum[i][j];
                if t >= k {
                    ret += *map.get(&(t - k)).unwrap_or(&0);
                }
                *map.entry(t).or_insert(0) += 1;
            }
        }
    }
    println!("{ret}")
}
