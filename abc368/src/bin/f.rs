use std::io::{stdout, Write};

use cpio::*;

const M: usize = 100001;
const RESULT: [u32; M] = {
    let mut mask = [1u32; M];
    let mut mex = [0u32; M];
    let mut i = 2;
    while i < M {
        mex[i] = mask[i].trailing_ones();
        let mut j = 2;
        while i * j < M {
            mask[i * j] |= 1 << mex[i];
            j += 1;
        }
        i += 1;
    }
    mex
};

fn main() {
    scan!(n: usize, a: [u32; n]);
    stdout()
        .write_all(
            ["Anna\n", "Bruno\n"]
                [(a.into_iter().fold(0, |s, v| s ^ RESULT[v as usize]) == 0) as usize]
                .as_bytes(),
        )
        .ok();
    stdout().flush().ok();
}
