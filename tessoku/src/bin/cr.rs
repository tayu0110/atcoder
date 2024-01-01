use std::sync::Mutex;

use proconio::*;

const MAX: usize = 100001;
static BUF: Mutex<[u32; MAX]> = Mutex::new([u32::MAX; MAX]);

fn main() {
    input! {n: usize, w: u32}

    let mut dp = BUF.lock().unwrap();
    dp[0] = 0;
    for _ in 0..n {
        input! {nw: u32, nv: usize}

        for i in (0..MAX).rev() {
            unsafe {
                if i + nv < MAX && *dp.get_unchecked(i) < w {
                    *dp.get_unchecked_mut(i + nv) =
                        (*dp.get_unchecked(i + nv)).min(dp.get_unchecked(i) + nw);
                }
            }
        }
    }

    println!(
        "{}",
        dp.into_iter()
            .enumerate()
            .rev()
            .skip_while(|&(_, d)| d > w)
            .next()
            .map(|v| v.0)
            .unwrap()
    )
}
