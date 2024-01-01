use math::divisors_enumeration;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [u64; n]}

    let mut cnt = vec![0; 1000_010];
    for a in a {
        for d in divisors_enumeration(a) {
            cnt[d as usize] += 1;
        }
    }

    println!(
        "{}",
        cnt.into_iter()
            .enumerate()
            .filter_map(|(i, c)| (c >= k).then_some(i))
            .max()
            .unwrap()
    )
}
