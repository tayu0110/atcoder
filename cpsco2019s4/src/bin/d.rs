use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut rle = vec![];
    for a in a {
        match rle.last_mut() {
            Some((cnt, r)) if *r == a => *cnt += 1,
            _ => rle.push((1, a)),
        }
    }

    let (mut l, mut r) = (0, n as i32);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut sum = 0;
        for &(cnt, _) in &rle {
            let mut now = cnt;
            while now > m {
                now -= m + 1;
                sum += 1;
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
