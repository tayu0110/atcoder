use proconio::*;

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize); n]}

    let (mut l, mut r) = (0, 1000000000000usize);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut sum = 0;
        for &(a, b) in &p {
            if a >= m {
                sum += b;
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
