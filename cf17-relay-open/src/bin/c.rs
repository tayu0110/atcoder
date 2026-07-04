use proconio::*;

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize); n]}

    let (mut l, mut r) = (0, 5000_000_000_000_000_000usize);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut cnt = 0;
        for &(w, d) in &p {
            if w <= m {
                cnt += (m - w) / d + 1;
            }
        }

        if k <= cnt {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{r}")
}
