use proconio::*;

fn main() {
    input! {n: usize, d: usize, mut r: [usize; n]}
    r.sort();

    let mut res = 0;
    for (i, &nr) in r.iter().enumerate() {
        let (mut lh, mut rh) = (i, n);
        while rh - lh > 1 {
            let m = (rh + lh) / 2;
            if r[m] - nr <= d {
                lh = m;
            } else {
                rh = m;
            }
        }

        let k = lh - i;
        res += k * k.saturating_sub(1) / 2;
    }

    println!("{}", res)
}
