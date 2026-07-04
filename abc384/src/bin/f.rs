use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut d = [0; 26];
    for i in 0..25 {
        let mask = 1 << i;
        let mut memo = FxHashMap::default();
        for &a in &a {
            let (cnt, sum) = memo.entry((mask - a % mask) % mask).or_insert((0, 0));
            *cnt += 1;
            *sum += a;

            let (cnt, sum) = memo.entry(a & (mask - 1)).or_insert((0, 0));
            d[i] += *cnt * a + *sum;
        }
    }

    println!(
        "{}",
        d.windows(2)
            .enumerate()
            .map(|(i, d)| (d[0] - d[1]) >> i)
            .sum::<usize>()
    );
}
