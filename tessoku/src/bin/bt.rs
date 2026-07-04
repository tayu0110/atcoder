use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {h: usize, w: usize, k: usize, c: [marker::Bytes; h]}

    let mut res = 0;
    for i in 0u32..1 << h {
        if i.count_ones() > k as u32 {
            continue;
        }

        let mut buf = vec![];
        for j in 0..w {
            let mut cnt = 0;
            for k in 0..h {
                if i & (1 << k) == 0 {
                    cnt += (c[k][j] == b'.') as u32;
                }
            }
            buf.push(cnt);
        }
        buf.sort_unstable_by_key(|c| Reverse(*c));
        res = res.max(
            h as u32 * w as u32
                - (buf.iter().sum::<u32>()
                    - buf[..w.min(k - i.count_ones() as usize)]
                        .iter()
                        .sum::<u32>()),
        );
    }
    println!("{res}")
}
