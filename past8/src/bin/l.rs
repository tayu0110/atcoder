use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [i32; n]}

    let mut cum = vec![0; n + 1];
    for (i, a) in a.into_iter().enumerate() {
        cum[i + 1] = cum[i] + a as i64;
    }

    cum.sort_unstable();
    let (mut l, mut r) = (-1, 300_000_000_000_000);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut sum = 0;
        {
            let (mut pl, mut pr) = (0, 0);
            while pr < cum.len() && sum < k {
                while pr < cum.len() && cum[pr] - cum[pl] <= m {
                    pr += 1;
                }
                sum += pr - pl - 1;
                pl += 1;
            }
            let d = (cum.len() - pl).saturating_sub(1);
            sum += d * (d + 1) / 2;
        }

        if sum >= k {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{r}");
}
