use proconio::*;
use rustc_hash::FxHashMap;

fn transpose(h: usize, w: usize, s: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            new[j][i] = s[i][j];
        }
    }
    new
}

#[fastout]
fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {mut h: usize, mut w: usize, mut s: [marker::Bytes; h]}

        if h < w {
            s = transpose(h, w, s);
            (h, w) = (w, h);
        }

        let mut cum = vec![vec![0; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                cum[i + 1][j + 1] = cum[i + 1][j];
                if s[i][j] == b'#' {
                    cum[i + 1][j + 1] += 1;
                } else {
                    cum[i + 1][j + 1] -= 1;
                }
            }
        }
        for i in 0..h {
            for j in 0..w + 1 {
                cum[i + 1][j] += cum[i][j];
            }
        }

        let mut res = 0usize;
        for l in 0..w {
            for r in l + 1..w + 1 {
                let mut memo = FxHashMap::default();
                for i in 0..h + 1 {
                    let entry = memo.entry(cum[i][r] - cum[i][l]).or_insert(0);
                    res += *entry;
                    *entry += 1;
                }
            }
        }

        println!("{res}")
    }
}
