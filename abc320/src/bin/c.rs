use itertools::Itertools;
use proconio::*;

fn main() {
    input! {m: usize, s: [marker::Chars; 3]}

    let mut t = vec![vec![vec![]; 10]; 3];
    for i in 0..3 {
        for j in 0..s[i].len() {
            let now = (s[i][j] as u8 - b'0') as usize;
            t[i][now].push(j);
        }
    }

    let mut res = usize::MAX;
    for i in 0..10 {
        if t.iter().any(|v| v[i].is_empty()) {
            continue;
        }

        for v in (0..3).permutations(3) {
            let mut now = 0;
            for l in 0..3 {
                let j = v[l];
                let mut min = usize::MAX;
                for k in 0..t[j][i].len() {
                    let d = now / m;
                    let mut e = d * m + t[j][i][k];
                    if l > 0 && e <= now {
                        e += s[j].len();
                    }

                    min = min.min(e);
                }

                now = min;
            }
            res = res.min(now);
        }
    }

    println!("{}", res as i64);
}
