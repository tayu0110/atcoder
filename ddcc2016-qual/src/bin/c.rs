use proconio::*;
use std::mem::swap;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let gcd = |mut x: usize| -> usize {
        let mut k = k;
        while k != 0 {
            x %= k;
            swap(&mut x, &mut k);
        }
        x
    };

    let mut a = a.into_iter().map(gcd).collect::<Vec<_>>();
    a.sort();
    let mut rle = vec![];
    for a in a {
        match rle.last_mut() {
            Some((cnt, p)) if *p == a => *cnt += 1,
            _ => rle.push((1, a)),
        }
    }

    let (cnt, a) = rle.into_iter().unzip::<usize, usize, Vec<_>, Vec<_>>();
    let len = a.len();
    let mut res = 0;
    for i in 0..len {
        if a[i] * a[i] % k == 0 {
            res += cnt[i] * cnt[i].saturating_sub(1) / 2;
        }
        for j in i + 1..len {
            if a[i] * a[j] % k == 0 {
                res += cnt[i] * cnt[j];
            }
        }
    }

    println!("{}", res)
}
