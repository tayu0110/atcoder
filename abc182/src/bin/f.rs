#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut x: i64, a: [i64; n]}

    let mut dp = vec![std::collections::HashMap::new(); n];
    dp[0].insert(x, 1usize);

    for i in 0..n-1 {
        let mut map = std::collections::HashMap::new();
        let d = a[i+1] / a[i];
        for (k, v) in &dp[i] {
            if *k % d != 0 {
                *map.entry(*k / d + 1).or_insert(0) += *v;
            }
            *map.entry(*k / d).or_insert(0) += *v;
        }

        std::mem::swap(&mut dp[i+1], &mut map);
    }

    let res = dp[n-1].values().sum::<usize>();
    println!("{}", res);
}
