use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, m: u32, a: [u32; n]}

    let mut memo = vec![FxHashMap::default(); 11];
    for &(mut a) in &a {
        a %= m;
        for j in 0..11 {
            *memo[j].entry(a).or_insert(0) += 1;
            a = (a as u64 * 10 % m as u64) as u32;
        }
    }

    let mut ret = 0usize;
    for a in a {
        let len = a.to_string().len();
        ret += *memo[len].get(&((m - a % m) % m)).unwrap_or(&0);
    }

    println!("{ret}")
}
