use proconio::*;
use rustc_hash::FxHashMap;

const M: usize = 998244353;

fn main() {
    input! {mut h: usize, mut w: usize, mut s: [marker::Bytes; h]}

    if w > h {
        let mut new = vec![vec![b'?'; h]; w];
        for i in 0..h {
            for j in 0..w {
                new[j][i] = s[i][j];
            }
        }
        (h, w) = (w, h);
        s = new;
    }

    let mut memo = FxHashMap::default();
    memo.insert(0, 1);
    let base = 10usize.pow(w as u32 - 1);
    for i in 0..h {
        for j in 0..w {
            let mut next = FxHashMap::default();
            for (&k, &v) in memo.iter() {
                let up = k / base + b'0' as usize;
                let prev = k % 10 + b'0' as usize;
                for c in b'1'..=b'3' {
                    if up != c as usize
                        && (j == 0 || prev != c as usize)
                        && (s[i][j] == c || s[i][j] == b'?')
                    {
                        let up = (up - b'0' as usize) * base;
                        let key = (k - up) * 10 + c as usize - b'0' as usize;
                        let entry = next.entry(key).or_insert(0);
                        *entry += v;
                        *entry %= M;
                    }
                }
            }
            memo = next;
        }
    }

    println!("{}", memo.into_values().fold(0, |s, v| (s + v) % M))
}
