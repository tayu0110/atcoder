use proconio::*;

fn main() {
    input! {n: usize, k: u128, a: [u128; n]}

    let (mut l, mut r) = (0, u128::MAX >> 50);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut k = k;
        let mut bad = false;
        for i in 0..n {
            let diff = m.saturating_sub(a[i]).div_ceil(i as u128 + 1);
            if k < diff {
                bad = true;
                break;
            }
            k -= diff;
        }

        if bad {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", l)
}
