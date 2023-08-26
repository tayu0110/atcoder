use proconio::*;

fn main() {
    input! {n: usize, mut k: i64, mut a: [i64; n]}
    a.sort();
    a.reverse();

    let (mut l, mut r) = (-1, 10_000_000_000i64);
    while r - l > 1 {
        let m = (r + l) / 2;
        let sum = a.iter().fold(0, |s, &v| s + (v - m).max(0));
        if sum < k {
            r = m;
        } else {
            l = m;
        }
    }

    let mut res = 0;
    for a in a.iter_mut() {
        if *a <= r {
            break;
        }

        let diff = (*a - r).min(k);
        let m = *a - diff;
        k -= diff;
        res += *a * (*a + 1) / 2 - m * (m + 1) / 2;
        *a -= diff;
    }
    eprintln!("l: {}, r: {}, res: {}", l, r, res);

    if l >= 0 {
        a.sort();
        a.reverse();
        for a in a {
            if a <= l {
                break;
            }

            let diff = (a - l).min(k);
            let m = a - diff;
            k -= diff;
            res += a * (a + 1) / 2 - m * (m + 1) / 2;
        }
    }

    println!("{}", res);
}
