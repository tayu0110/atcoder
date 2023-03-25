use std::arch::x86_64::{
    _mm256_load_si256, _mm256_max_epi16, _mm256_min_epi16, _mm256_set1_epi16, _mm256_store_si256,
};

use proconio::input;

#[repr(align(32))]
struct Aligned {
    val: [[i16; 2048]; 2048],
}

static mut DP: Aligned = Aligned {
    val: [[std::i16::MAX; 2048]; 2048],
};

fn main() {
    use std::io::Write;
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());

    input! {n: usize, m: usize}

    let size = (n + 15) >> 4 << 4;

    let dp = unsafe { &mut DP.val };

    for i in 0..n {
        dp[i][i] = i as i16 + 1;
    }

    for _ in 0..m {
        input! {a: usize, b: usize}
        dp[a - 1][b - 1] = std::cmp::max(a as i16, b as i16);
    }

    unsafe {
        for k in 0..n {
            for i in 0..n {
                if dp[i][k] < std::i16::MAX {
                    let ik = _mm256_set1_epi16(dp[i][k]);

                    let mut kj = dp[k].as_ptr();
                    let mut ij = dp[i].as_ptr();
                    for _ in 0..size >> 4 {
                        let max = _mm256_max_epi16(_mm256_load_si256(kj as _), ik);
                        let min = _mm256_min_epi16(_mm256_load_si256(ij as _), max);
                        _mm256_store_si256(ij as *mut _, min);

                        kj = kj.add(16);
                        ij = ij.add(16);
                    }
                    // for (kj, ij) in dp[k]
                    //     .chunks_exact(16)
                    //     .zip(dp[i].chunks_exact(16))
                    //     .take(size >> 4)
                    // {
                    //     let max = _mm256_max_epi16(_mm256_load_si256(kj.as_ptr() as _), ik);
                    //     let min = _mm256_min_epi16(_mm256_load_si256(ij.as_ptr() as _), max);
                    //     _mm256_store_si256(ij.as_ptr() as *mut _, min);
                    // }
                }
            }
        }
    }

    input! {q: usize}
    for _ in 0..q {
        input! {s: usize, t: usize}
        if dp[s - 1][t - 1] == std::i16::MAX {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{}", dp[s - 1][t - 1]).unwrap();
        }
    }
}
