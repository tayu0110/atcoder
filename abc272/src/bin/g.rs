use std::time::Instant;

use math::MathInt;
use proconio::*;
use rand::{thread_rng, Rng};

fn main() {
    let now = Instant::now();
    input! {n: usize, a: [usize; n]}

    let mut rng = thread_rng();
    let mut t = 0;
    while now.elapsed().as_millis() < 1900 || t < 100 {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i == j {
            continue;
        }

        for div in a[i]
            .abs_diff(a[j])
            .divisors()
            .into_iter()
            .filter(|&d| d > 2)
        {
            let mut a = a.iter().map(|a| a % div).collect::<Vec<_>>();
            a.sort_unstable();
            let mut max = 0;
            let mut rle = None;
            for a in a {
                match rle.as_mut() {
                    Some((p, cnt)) if *p == a => {
                        *cnt += 1;
                    }
                    Some((_, cnt)) => {
                        max = max.max(*cnt);
                        rle = Some((a, 1));
                    }
                    None => {
                        rle = Some((a, 1));
                    }
                }
            }

            if let Some((_, cnt)) = rle {
                max = max.max(cnt);
            }

            if max * 2 > n {
                println!("{div}");
                return;
            }
        }

        t += 1;
    }

    println!("-1");
}
