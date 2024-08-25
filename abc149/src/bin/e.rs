#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, m: i64, mut a: [i64; n]}

    a.sort();
    let mut sum = a.iter().scan(0, |s, v| {
        *s += *v;
        Some(*s)
    }).collect_vec();
    sum.insert(0, 0);

    let (mut l, mut r) = (-1, 1_000_000_000_000_i64);
    let mut rk = 0;
    let mut res = 0;
    while r - l > 1 {
        let mid = (r + l) / 2;
        let mut k = 0;
        let mut score = 0;

        for v in &a {
            let target = mid - *v;
            let (mut l, mut r) = (-1, n as i64);
            while r - l > 1 {
                let m = (r + l) / 2;
                if a[m as usize] >= target {
                    r = m;
                } else {
                    l = m;
                }
            }
            
            k += n as i64 - r;
            score += sum[n] - sum[r as usize] + *v * (n as i64 - r);
            // eprintln!("\tl: {}, r: {}, sum[n]-sum[r]: {}, s: {}", l, r, sum[n] - sum[r as usize], sum[n] - sum[r as usize] + *v * (n as i64 - r));
        }

        if k <= m {
            r = mid;
            res = score;
            rk = k;
        } else {
            l = mid;
        }

        // eprintln!("l: {}, r: {}, k: {}, mid: {}, res: {}, score: {}", l, r, k, mid, res, score);
    }

    if rk < m {
        res += (m - rk) * l;
    }
    println!("{}", res);
}
