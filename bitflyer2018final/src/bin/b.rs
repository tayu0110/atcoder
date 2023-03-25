use proconio::*;

fn main() {
    input! {n: usize, q: usize, x: [i64; n], p: [(i64, i64); q]}

    let mut cum = vec![0];
    for i in 0..n {
        let next = cum[i] + x[i];
        cum.push(next);
    }

    for (c, d) in p {
        let r = {
            let (mut l, mut r) = (-1, n as i32);
            while r - l > 1 {
                let m = (r + l) / 2;
                if x[m as usize] <= c + d {
                    l = m;
                } else {
                    r = m;
                }
            }
            l
        };
        let l = {
            let (mut l, mut r) = (-1, n as i32);
            while r - l > 1 {
                let m = (r + l) / 2;
                if c <= x[m as usize] + d {
                    r = m;
                } else {
                    l = m;
                }
            }
            r
        };
        let mid = {
            let (mut l, mut r) = (-1, n as i32);
            while r - l > 1 {
                let m = (r + l) / 2;
                if x[m as usize] < c {
                    l = m;
                } else {
                    r = m;
                }
            }
            l
        };

        if l == n as i32 {
            println!("{}", d * n as i64);
        } else if r == -1 {
            println!("{}", d * n as i64);
        } else {
            let hi = n as i64 - 1 - r as i64;
            let lo = l as i64;

            if mid < l {
                println!(
                    "{}",
                    (hi + lo) * d + cum[r as usize + 1] - cum[l as usize] - c * (r + 1 - l) as i64
                )
            } else if r < mid {
                println!(
                    "{}",
                    (hi + lo) * d + c * (r + 1 - l) as i64 - cum[r as usize + 1] + cum[l as usize]
                )
            } else {
                println!(
                    "{}",
                    (hi + lo) * d
                        + (c * (mid + 1 - l) as i64 - cum[mid as usize + 1] + cum[l as usize])
                        + (cum[r as usize + 1] - cum[mid as usize + 1] - c * (r - mid) as i64)
                )
            }
        }
    }
}
