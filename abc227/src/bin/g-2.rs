use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {n: usize, k: usize}
    let sq = (n as f64).sqrt().ceil() as usize + 1;

    let mut primes = vec![];
    let size = std::cmp::max(k, sq);
    let mut lo = vec![std::usize::MAX; size + 1];
    for i in 2..=size {
        if lo[i] == std::usize::MAX {
            primes.push(i);
            for j in (1..=size).take_while(|&j| i * j <= size) {
                lo[i * j] = i;
            }
        }
    }

    let mut div = std::collections::HashMap::new();
    for i in 2..=k {
        let mut now = i;
        while now > 1 {
            *div.entry(lo[now]).or_insert(0) += 1;
            now /= lo[now];
        }
    }

    let m = n - k + 1;
    let mut hi = (m..=n).collect::<Vec<_>>();
    let mut map = std::collections::HashMap::new();
    for &k in &primes {
        let start = (m + k - 1) / k * k;
        for j in (start - m..hi.len()).step_by(k) {
            while hi[j] % k == 0 {
                *map.entry(k).or_insert(0usize) += 1;
                hi[j] /= k;
            }
        }
    }

    for hi in hi.into_iter().filter(|&v| v != 1) {
        *map.entry(hi).or_insert(0) += 1;
    }

    let mut res = 1;
    for (k, v) in map {
        let v = v - *div.get(&k).unwrap_or(&0);
        res *= v + 1;
        res %= MOD;
    }

    println!("{}", res);
}
