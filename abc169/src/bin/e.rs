use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let (mut l, mut r) = (0, 1000000000000);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut cnt = 0;
        for &(a, _) in &p {
            if m >= a {
                cnt += 1;
            }
        }

        if cnt >= (n + 1) / 2 {
            r = m;
        } else {
            l = m;
        }
    }
    let res = r;

    let (mut l, mut r) = (0, 1000000000000);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut cnt = 0;
        for &(_, b) in &p {
            if m <= b {
                cnt += 1;
            }
        }

        if cnt >= (n + 1) / 2 {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l - res + 1);
}
