use proconio::*;

fn main() {
    input! {n: usize, q: usize, a: [i64; n], k: [i64; q]}

    for k in k {
        let (mut l, mut r) = (-1, n as i64);
        while r - l > 1 {
            let m = (r + l) / 2;
            if a[m as usize] - (m + 1) < k {
                l = m;
            } else {
                r = m;
            }
        }

        if r == 0 {
            println!("{}", k);
        } else {
            println!("{}", k + r);
        }
    }
}
