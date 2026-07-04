use cpio::*;

fn main() {
    scan!(n: usize, mut p: [(u32, u32); n]);

    p.sort_unstable_by_key(|&(l, r)| ((l as u64) << 32) | r.wrapping_neg() as u64);
    let mut p = p
        .into_iter()
        .enumerate()
        .map(|v| v.0 as u64 | ((v.1 .1.wrapping_neg() as u64) << 32))
        .collect::<Vec<_>>();
    p.sort_unstable();
    let p = p.into_iter().map(|v| v as u32).collect::<Vec<_>>();

    let mut lis = vec![u32::MAX; n];
    for p in p {
        let pos = lis.partition_point(|&l| l < p);
        lis[pos] = p;
    }
    println!("{}", lis.into_iter().take_while(|&l| l != u32::MAX).count())
}
