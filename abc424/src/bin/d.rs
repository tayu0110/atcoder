use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {h: usize, w: usize, s: [marker::Bytes; h]}

        let mut dp = [usize::MAX; 1 << 7];
        dp[0] = 0;
        for i in 0..h {
            let mut mask = 0usize;
            for j in 0..w {
                mask <<= 1;
                if s[i][j] == b'#' {
                    mask |= 1;
                }
            }

            let mut next = [usize::MAX; 1 << 7];
            for j in 0..1 << w {
                if dp[j] == usize::MAX {
                    continue;
                }
                for k in 0..1 << w {
                    if k | mask != mask {
                        continue;
                    }

                    let mut bad = false;
                    for l in 1..w {
                        let a = (j >> (l - 1)) & 1 == 0;
                        let b = (j >> l) & 1 == 0;
                        let c = (k >> (l - 1)) & 1 == 0;
                        let d = (k >> l) & 1 == 0;
                        if !(a | b | c | d) {
                            bad = true;
                        }
                    }
                    if !bad {
                        next[k] = next[k].min(dp[j] + (mask ^ k).count_ones() as usize);
                    }
                }
            }
            dp = next;
        }

        println!("{}", dp.iter().min().unwrap());
    }
}
