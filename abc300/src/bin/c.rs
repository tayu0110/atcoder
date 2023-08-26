use itertools::Itertools;
use proconio::*;

fn main() {
    input! {h: usize, w: usize, c: [marker::Chars; h]}

    let sharp = '#';
    let mut res = vec![0; h.min(w) + 1];
    for i in 1..h - 1 {
        'mid: for j in 1..w - 1 {
            if c[i][j] == '.' {
                continue;
            }

            if c[i - 1][j - 1] == sharp
                && c[i - 1][j + 1] == sharp
                && c[i + 1][j - 1] == sharp
                && c[i + 1][j + 1] == sharp
            {
                for k in 1..=h.min(w) {
                    if i >= k && j >= k {
                        if c[i - k][j - k] == '.' {
                            res[k - 1] += 1;
                            continue 'mid;
                        }
                    } else {
                        res[k - 1] += 1;
                        continue 'mid;
                    }
                }
            }
        }
    }

    println!("{}", res.into_iter().skip(1).join(" "));
}
