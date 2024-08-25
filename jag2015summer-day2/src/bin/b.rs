use proconio::*;

fn main() {
    input! {n: u64, k: u64}

    let mut now = 0;
    for _ in 0..n - 1 {
        let (mut l, mut r) = (now, 1000_000_000_000_000_010);
        while r - l > 1 {
            let m = (r + l) / 2;
            let f = m - m / k;
            if now + 1 <= f {
                r = m;
            } else {
                l = m;
            }
        }

        now = r;
        // eprintln!("now: {now}");
    }

    println!("{now}");
}
