use std::mem::take;

use itertools::Itertools;
use num::integer::Roots;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n], b: [usize; m]}

    let mut cum = vec![0usize; n];
    let sq = n.sqrt();
    {
        let mut a = a.chunks_mut(sq).collect::<Vec<_>>();
        let mut bucket = cum.chunks_mut(sq).collect::<Vec<_>>();
        for b in b {
            let (bi, offset) = (b / sq, b % sq);
            for i in 0..bucket[bi].len() - 1 {
                bucket[bi][i + 1] = bucket[bi][i].wrapping_add(bucket[bi][i + 1]);
            }
            a[bi]
                .iter_mut()
                .zip(bucket[bi].iter())
                .for_each(|(a, b)| *a += b);
            bucket[bi].fill(0);

            let mut ball = take(&mut a[bi][offset]);

            if ball >= n {
                let ave = ball / n;
                bucket
                    .iter_mut()
                    .for_each(|b| b[0] = b[0].wrapping_add(ave));
                ball %= n;
            }

            if ball > 0 && offset + 1 < bucket[bi].len() {
                bucket[bi][offset + 1] = bucket[bi][offset + 1].wrapping_add(1);
                if offset + 1 + ball < bucket[bi].len() {
                    bucket[bi][offset + 1 + ball] = bucket[bi][offset + 1 + ball].wrapping_sub(1);
                }
                ball -= (ball).min(bucket[bi].len() - 1 - offset);
            }

            let mut now = (bi + 1) % bucket.len();
            while ball > 0 {
                bucket[now][0] = bucket[now][0].wrapping_add(1);
                if ball < bucket[now].len() {
                    bucket[now][ball] = bucket[now][ball].wrapping_sub(1);
                }
                ball = ball.saturating_sub(bucket[now].len());
                now = (now + 1) % bucket.len();
            }
        }

        for b in bucket.iter_mut() {
            for i in 0..b.len() - 1 {
                b[i + 1] = b[i + 1].wrapping_add(b[i]);
            }
        }
    }

    a.iter_mut().zip(cum).for_each(|(a, b)| *a += b);

    println!("{}", a.iter().join(" "))
}
