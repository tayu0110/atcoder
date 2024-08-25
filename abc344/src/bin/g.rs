use std::arch::x86_64::{
    _mm256_add_epi64, _mm256_blend_epi32, _mm256_cmpgt_epi64, _mm256_load_si256,
    _mm256_movemask_epi8, _mm256_mul_epi32, _mm256_set1_epi64x, _mm256_setzero_si256,
    _mm256_shuffle_epi32,
};

use proconio::*;

const M: u32 = (1 << 31) - 1;

#[repr(align(32))]
struct AlignedArray {
    arr: [i32; 5000],
}

#[target_feature(enable = "avx2")]
unsafe fn solve(n: usize, p: Vec<(i32, i32)>, q: usize, mut g: u32, ra: u32, rb: u64) -> u64 {
    let x = &mut AlignedArray { arr: [0; 5000] }.arr[..n];
    let y = &mut AlignedArray { arr: [0; 5000] }.arr[..n];
    for (i, (nx, ny)) in p.into_iter().enumerate() {
        x[i] = nx;
        y[i] = ny;
    }

    let mut res = 0u64;
    let mut ng = [0u32; 3];
    for _ in 0..q {
        g = ((g as u64 * 48271) % M as u64) as u32;
        ng[0] = g;
        g = ((g as u64 * 48271) % M as u64) as u32;
        ng[1] = g;
        g = ((g as u64 * 48271) % M as u64) as u32;
        ng[2] = g;

        let a = (ng[0] % (2 * ra + 1)).wrapping_sub(ra);
        let b = ((ng[1] as u64 * ((1 << 31) - 1) + ng[2] as u64) % (2 * rb + 1)).wrapping_sub(rb);

        let va = _mm256_set1_epi64x(a as i64);
        let vb = _mm256_set1_epi64x(b as i64);

        let mut xi = x.chunks_exact(8);
        let mut yi = y.chunks_exact(8);
        for (x, y) in xi.by_ref().zip(yi.by_ref()) {
            let x = _mm256_load_si256(x.as_ptr() as _);
            let y = _mm256_load_si256(y.as_ptr() as _);

            let r = _mm256_add_epi64(vb, _mm256_mul_epi32(va, x));
            let mask = _mm256_cmpgt_epi64(
                r,
                _mm256_blend_epi32::<0b10101010>(y, _mm256_setzero_si256()),
            );
            let maska = _mm256_movemask_epi8(mask) as u64;

            let r = _mm256_add_epi64(
                vb,
                _mm256_mul_epi32(va, _mm256_shuffle_epi32::<0b00_11_00_01>(x)),
            );
            let mask = _mm256_cmpgt_epi64(
                r,
                _mm256_shuffle_epi32::<0b00_11_00_01>(_mm256_blend_epi32::<0b01010101>(
                    y,
                    _mm256_setzero_si256(),
                )),
            );
            let maskb = _mm256_movemask_epi8(mask) as u64;
            res += 8 - ((maska | maskb.wrapping_shl(32)).count_ones() >> 3) as u64;
        }

        for (&x, &y) in xi.remainder().into_iter().zip(yi.remainder()) {
            res += (y as i64 >= a as i32 as i64 * x as i64 + b as i64) as u64;
        }
    }
    res
}

fn main() {
    input! {n: usize, p: [(i32, i32); n], q: usize, mut g: u32, ra: u32, rb: u64}

    let res = unsafe { solve(n, p, q, g, ra, rb) };
    println!("{res}");
}
