use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
        cum[i + 1] %= k;
    }

    let mut res = 0usize;
    let mut map = HashMap::new();
    for i in 0..n + 1 {
        if i >= k {
            let t = (i - cum[i - k]) % k;
            *map.entry(t).or_insert(0) -= 1;
        }

        let t = (i + k - cum[i]) % k;
        res += *map.get(&t).unwrap_or(&0);
        *map.entry(t).or_insert(0) += 1;
    }

    println!("{}", res)
}
