use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize, n: usize, p: [(u32, u32); n]}

    let mut d = vec![0u32; t + 1];
    for (l, r) in p {
        d[l as usize] += 1;
        d[r as usize] = d[r as usize].wrapping_sub(1);
    }
    for i in 0..t {
        d[i + 1] = d[i + 1].wrapping_add(d[i]);
    }

    println!("{}", d[..t].iter().join("\n"))
}
