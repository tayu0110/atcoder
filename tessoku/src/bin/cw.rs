use proconio::*;

fn main() {
    input! {n: usize, mut p: [(u32, u32); n]}

    p.sort_unstable_by(|v, w| if v.0 == w.0 { w.1.cmp(&v.1) } else { v.cmp(&w) });
    let (_, mut p): (Vec<u32>, Vec<u32>) = p.into_iter().unzip();

    for i in 1..n {
        let now = unsafe {
            let res = *p.get_unchecked(i);
            *p.get_unchecked_mut(i) = u32::MAX;
            res
        };
        let pos = p[..=i].partition_point(|&l| l < now);
        unsafe {
            *p.get_unchecked_mut(pos) = now;
        }
    }

    println!("{}", p.partition_point(|&l| l < u32::MAX))
}
