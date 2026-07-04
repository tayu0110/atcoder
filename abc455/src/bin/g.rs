use proconio::*;

fn solve(a: &[usize], b: usize) {
    let n = a.len();
    for m in (1..).take_while(|i| i * b <= n) {
        let block = m * b;
        let mut cnt = vec![0; n + 1];
        let mut diff = 0;
        for i in 0..block {
            cnt[a[i]] += 1;
            if cnt[a[i]] == 1 {
                diff += 1;
            }
            if cnt[a[i]] == m {
                diff -= 1;
            } else if cnt[a[i]] - 1 == m {
                diff += 1;
            }
        }
        let mut ret = (diff == 0) as usize;
        for i in block..n {
            cnt[a[i - block]] -= 1;
            if cnt[a[i - block]] == m {
                diff -= 1;
            } else if cnt[a[i - block]] + 1 == m {
                diff += 1;
            }

            cnt[a[i]] += 1;
            if cnt[a[i]] == 1 {
                diff += 1;
            }
            if cnt[a[i]] == m {
                diff -= 1;
            } else if cnt[a[i]] - 1 == m {
                diff += 1;
            }

            if diff == 0 {
                ret += 1;
            }
        }
    }

    todo!()
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, k: usize, a: [usize; n], b: [usize; k]}

        for b in b {
            if b * b >= n {
                solve(&a, b);
            } else {
                todo!()
            }
        }
    }
}
