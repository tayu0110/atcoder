use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let (mut l, mut r) = (0, 1_000_000_000_000);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut sum = 0;
        for &a in &a {
            if a % m == 0 {
                sum += a / m - 1;
            } else {
                sum += (a + m - 1) / m - 1;
            }
        }

        if sum <= k {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r)
}
