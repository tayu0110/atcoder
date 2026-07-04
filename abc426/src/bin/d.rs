use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut ret = vec![];
    for _ in 0..t {
        input! {n: usize, s: marker::Bytes}

        let mut rle = vec![];
        for s in s {
            match rle.last_mut() {
                Some((p, cnt)) if *p == (s - b'0') => *cnt += 1,
                _ => rle.push((s - b'0', 1)),
            }
        }

        let mut cnt = [0; 2];
        for &(b, c) in &rle {
            cnt[b as usize] += c;
        }

        let mut res = usize::MAX;
        for (b, c) in rle {
            res = res.min(n - c + (cnt[b as usize] - c));
        }
        ret.push(res);
    }

    println!("{}", ret.iter().join("\n"))
}
