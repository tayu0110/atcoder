#![allow(unused_imports)]
use proconio::input;

fn main() {
    input! {n: u32, m: usize, p: [(u32, u32, u32); m]}

    let v = p
        .into_iter()
        .map(|(a, b, c)| (1 << (a - 1)) | (1 << (b - 1)) | (1 << (c - 1)))
        .collect::<Vec<_>>();
    let mut res = 0;
    'base: for i in 0u32..(1 << n) {
        let mut k = 0u32;

        for &c in &v {
            let nc = i & c;
            if nc == c {
                continue 'base;
            }

            if nc.count_ones() < 2 {
                continue;
            }

            k |= c ^ nc;
        }

        res = std::cmp::max(res, k.count_ones());
    }

    println!("{}", res);
}
