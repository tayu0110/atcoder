use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {q: usize, p: [(usize, usize, usize); q]}

    let mut left = vec![0; q + 2];
    let mut right = vec![0; q + 2];
    let mut quant = vec![0; q + 2];
    quant[0] = 1;
    quant[1] = 1;
    let mut ans = vec![];
    let mut caches = vec![BTreeSet::new(); 2];
    for (i, (l, r, x)) in p.into_iter().enumerate().map(|(i, p)| (i + 2, p)) {
        left[i] = l;
        right[i] = r;
        quant[i] = quant[l] + quant[r];

        let mut cache = BTreeSet::new();
        let mut sum = 0;
        let mut now = i;
        while now > 1 {
            if let Some(&(s, n)) = caches
                .get(now)
                .and_then(|set| set.range(..=(x - sum, usize::MAX)).next_back())
                .filter(|&&(s, n)| {
                    (x > sum + quant[left[now]] && s > quant[left[now]])
                        || (x <= sum + quant[left[now]] && x - s - sum <= quant[n])
                })
            {
                now = n;
                sum += s;
            } else if x > sum + quant[left[now]] {
                sum += quant[left[now]];
                now = right[now];
                cache.insert((sum, now));
            } else {
                now = left[now];
            }
        }

        caches.push(cache);
        if now == 0 {
            ans.push(0u8);
        } else {
            ans.push(1u8);
        }
    }

    println!("{}", ans.iter().join("\n"))
}
