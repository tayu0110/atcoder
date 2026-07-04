use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, k: usize, x: usize, mut t: [usize; n]}
    t.insert(0, 0);

    let mut dp = vec![FxHashMap::default(); n + 1];
    dp[0].insert(0, 0);

    let mut mp = vec![];
    for i in 0..n {
        mp.extend(dp[i].iter().map(|v| (*v.0, *v.1)));
        for &(tm, value) in &mp {
            let mut sum = 0;
            for j in 1..=k {
                if i + j > n {
                    break;
                }
                sum += t[i + j];
                let next = t[i + j].max(tm);
                let entry = dp[i + j].entry(next + x).or_insert(usize::MAX);
                *entry = (*entry).min(value + next * j - sum);
            }
        }

        mp.clear();
    }

    println!("{}", dp[n].values().min().unwrap());
}
