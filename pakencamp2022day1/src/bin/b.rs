use proconio::*;

fn main() {
    input! {n: u128, m: u128}

    if n == 1 {
        println!("0");
        return;
    }

    if m == 1 {
        println!("-1");
        return;
    }

    let (mut l, mut r) = (0, 1_000_000_000_000);
    while r - l > 1 {
        let t = (r + l) / 2;
        let cnt = 1 + (m - 1) * (t - 1) + m;
        if cnt >= n {
            r = t;
        } else {
            l = t;
        }
    }

    println!("{}", r)
}
