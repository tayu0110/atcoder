use proconio::*;
use rustc_hash::FxHashMap;

const M: usize = 998244353;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.insert(0, 0);

    let mut cnt = FxHashMap::default();
    for &a in &a {
        *cnt.entry(a).or_insert(0) += 1;
    }
    cnt.retain(|k, v| *k <= *v);
    a.retain(|a| cnt.contains_key(a));

    let n = a.len();
    let keys = cnt.keys().copied().collect::<Vec<_>>();

    let mut position = FxHashMap::default();
    for (i, &a) in a.iter().enumerate() {
        position.entry(a).or_insert(vec![]).push(i);
    }
    let mut next = vec![FxHashMap::default(); n];
    for (i, &a) in a.iter().enumerate() {
        let ps = position.get(&a).unwrap();
        let pos = ps.binary_search(&i).unwrap();
        if (pos + a).saturating_sub(1) >= ps.len() {
            continue;
        }
        let pos = ps[(pos + a).saturating_sub(1)];

        for &k in &keys {
            if k == a {
                continue;
            }
            let ps = position.get(&k).unwrap();
            let pos = ps.partition_point(|&p| p < pos);
            if pos < ps.len() {
                next[i].insert(k, ps[pos]);
            }
        }
    }

    let mut dp = vec![0; n];
    dp[0] = 1;
    for i in 0..n {
        for &nv in next[i].values() {
            dp[nv] += dp[i];
            dp[nv] %= M;
        }
    }

    // eprintln!("dp: {dp:?}");
    println!(
        "{}",
        (dp[1..].iter().sum::<usize>() + M - next[0].len()) % M
    )
}
