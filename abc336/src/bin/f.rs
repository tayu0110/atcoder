use std::collections::VecDeque;

use proconio::*;
use rustc_hash::FxHashMap;

type HashMap<K, V> = FxHashMap<K, V>;

fn rotate(x: usize, y: usize, mut v: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let (h, w) = (v.len(), v[0].len());
    let mut moved = vec![vec![false; w]; h];
    for i in 1..h {
        for j in 1..w {
            let s = v[i + x - 1][j + y - 1];
            let t = v[h + x - i - 1][w + y - j - 1];
            if moved[i + x - 1][j + y - 1] {
                continue;
            }
            moved[i + x - 1][j + y - 1] = true;
            moved[h + x - i - 1][w + y - j - 1] = true;
            v[i + x - 1][j + y - 1] = t;
            v[h + x - i - 1][w + y - j - 1] = s;
        }
    }
    v
}

fn main() {
    input! {h: usize, w: usize, s: [[u8; w]; h]}

    let mut f: HashMap<Vec<Vec<u8>>, usize> = HashMap::default();
    let mut b: HashMap<Vec<Vec<u8>>, usize> = HashMap::default();
    let t = {
        let mut buf = vec![vec![0; w]; h];
        let mut cnt = 1;
        for i in 0..h {
            for j in 0..w {
                buf[i][j] = cnt;
                cnt += 1;
            }
        }
        buf
    };
    for (s, memo) in [(s, &mut f), (t, &mut b)] {
        let mut nt = VecDeque::new();
        nt.push_back((s, 0));
        while let Some((s, d)) = nt.pop_front() {
            if memo.contains_key(&s) {
                continue;
            }
            memo.entry(s.clone()).or_insert(d);
            if d == 10 {
                continue;
            }
            for i in 0..4 {
                let t = rotate(i & 1, (i & 2) >> 1, s.clone());
                if !memo.contains_key(&t) {
                    nt.push_back((t, d + 1));
                }
            }
        }
    }

    let mut res = usize::MAX;
    for (k, v) in f {
        if let Some(nv) = b.get(&k) {
            res = res.min(v + nv);
        }
    }

    println!("{}", res as i64)
}
