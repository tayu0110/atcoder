use proconio::input;
use std::arch::x86_64::*;

#[repr(C, align(32))]
struct AlignedArrayx8 {
    arr: [i32; 8],
}

#[target_feature(enable = "avx2")]
unsafe fn solve(n: usize, a: &[i32], b: &[i32]) -> i32 {
    let mut res = _mm256_setzero_si256();
    let mut k = 0;
    let rest_start = (n >> 3) << 3;

    const SHUFFLE: i32 = 0b00_11_10_01;
    let permute = _mm256_set_epi32(0, 3, 2, 1, 4, 7, 6, 5);

    for va in a.chunks_exact(8).take(n >> 3) {
        let a = _mm256_loadu_si256(va.as_ptr() as *const _);
        for vb in b.chunks_exact(8).take(n >> 3) {
            // 7, 6, 5, 4, 3, 2, 1, 0
            let b = _mm256_loadu_si256(vb.as_ptr() as *const _);
            res = _mm256_xor_si256(_mm256_add_epi32(a, b), res);

            // 4, 7, 6, 5, 0, 3, 2, 1
            let b = _mm256_shuffle_epi32(b, SHUFFLE);
            res = _mm256_xor_si256(_mm256_add_epi32(a, b), res);

            // 5, 4, 7, 6, 1, 0, 3, 2
            let b = _mm256_shuffle_epi32(b, SHUFFLE);
            res = _mm256_xor_si256(_mm256_add_epi32(a, b), res);

            // 6, 5, 4, 7, 2, 1, 0, 3
            let b = _mm256_shuffle_epi32(b, SHUFFLE);
            res = _mm256_xor_si256(_mm256_add_epi32(a, b), res);

            // 3, 2, 1, 0, 7, 6, 5, 4
            let b = _mm256_permutevar8x32_epi32(b, permute);
            res = _mm256_xor_si256(_mm256_add_epi32(a, b), res);

            // 0, 3, 2, 1, 4, 7, 6, 5
            let b = _mm256_shuffle_epi32(b, SHUFFLE);
            res = _mm256_xor_si256(_mm256_add_epi32(a, b), res);

            // 1, 0, 3, 2, 5, 4, 7, 6
            let b = _mm256_shuffle_epi32(b, SHUFFLE);
            res = _mm256_xor_si256(_mm256_add_epi32(a, b), res);

            // 2, 1, 0, 3, 6, 5, 4, 7
            let b = _mm256_shuffle_epi32(b, SHUFFLE);
            res = _mm256_xor_si256(_mm256_add_epi32(a, b), res);
        }
        for &a in va {
            for &b in b.iter().take(n).skip(rest_start) {
                k ^= a + b;
            }
        }
    }

    for na in a.iter().take(n).skip(rest_start) {
        for &nb in b.iter().take(n) {
            k ^= na + nb;
        }
    }

    let mut buf = AlignedArrayx8 { arr: [0; 8] };
    _mm256_store_si256(buf.arr.as_mut_ptr() as *mut _, res);
    buf.arr.into_iter().fold(0, |s, v| s ^ v) ^ k
}

fn main() {
    input! {n: usize, a: [i32; n], b: [i32; n]}

    println!("{}", unsafe { solve(n, &a, &b) });
}
