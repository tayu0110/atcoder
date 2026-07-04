use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: usize, a: [usize; n], e: [(usize, usize); m]}

    let mut ng = vec![0; n];
    for (x, y) in e {
        ng[x - 1] |= 1 << (y - 1);
        ng[y - 1] |= 1 << (x - 1);
    }

    let mut ok = vec![false; 1 << n];
    for i in 1..1 << n {
        let mut sum = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                if i & ng[j] == 0 {
                    sum = a[j].saturating_add(sum);
                } else {
                    sum = usize::MAX;
                }
            }
        }

        if sum <= s {
            ok[i] = true;
        }
    }

    let mut dp = vec![u8::MAX; 1 << n];
    dp[0] = 0;
    for i in 0..1 << n {
        if dp[i] < u8::MAX {
            let orig = ((1 << n) - 1) ^ i;
            let lsb = orig & orig.wrapping_neg();
            let orig = orig ^ lsb;
            let mut now = orig;
            while {
                if ok[now | lsb] {
                    dp[i | now | lsb] = dp[i | now | lsb].min(dp[i] + 1);
                }
                now > 0
            } {
                now -= 1;
                now &= orig
            }
        }
    }

    println!("{}", dp[(1 << n) - 1])
}
