use proconio::*;
use std::{
    arch::x86_64::{_mm256_add_epi32, _mm256_loadu_si256, _mm256_storeu_si256},
    sync::Mutex,
};

const N: usize = 1504;

static GRID: Mutex<[[u32; N]; N]> = Mutex::new([[0; N]; N]);

#[fastout]
fn main() {
    input! {n: usize}

    let mut grid = GRID.lock().unwrap();
    for _ in 0..n {
        input! {x: usize, y: usize}
        grid[y][x] += 1;
    }

    for i in 0..N - 1 {
        for (v, w) in grid[i].chunks_exact(8).zip(grid[i + 1].chunks_exact(8)) {
            unsafe {
                let res = _mm256_add_epi32(
                    _mm256_loadu_si256(v.as_ptr() as _),
                    _mm256_loadu_si256(w.as_ptr() as _),
                );
                _mm256_storeu_si256(w.as_ptr() as _, res);
            }
        }
    }
    for i in 0..N {
        for j in 0..N - 1 {
            grid[i][j + 1] += grid[i][j];
        }
    }

    input! {q: usize}
    for _ in 0..q {
        input! {a: usize, b: usize, c: usize, d: usize}
        println!(
            "{}",
            grid[d][c] + grid[b - 1][a - 1] - grid[b - 1][c] - grid[d][a - 1]
        )
    }
}
