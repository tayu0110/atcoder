use std::arch::x86_64::{_mm256_cmpeq_epi16, _mm256_loadu_si256, _mm256_movemask_epi8};

use proconio::*;

#[target_feature(enable = "avx2", enable = "bmi1")]
unsafe fn solve(n: usize, a: Vec<Vec<u16>>) -> u32 {
    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut sum = 0;
            let mut b = a[j].chunks_exact(16);
            let mut a = a[i].chunks_exact(16);
            for (a, b) in a.by_ref().zip(b.by_ref()) {
                let a = _mm256_loadu_si256(a.as_ptr() as _);
                let b = _mm256_loadu_si256(b.as_ptr() as _);
                let mask = _mm256_cmpeq_epi16(a, b);
                let mask = _mm256_movemask_epi8(mask);
                sum += mask.count_ones() >> 1;
            }

            sum += a
                .remainder()
                .into_iter()
                .zip(b.remainder().into_iter())
                .map(|(a, b)| (a == b) as u32)
                .sum::<u32>();

            res += sum & 1;
        }
    }
    res
}

fn main() {
    input! {n: usize, m: usize, a: [[u16; m]; n]}

    println!("{}", unsafe { solve(n, a) })
}
