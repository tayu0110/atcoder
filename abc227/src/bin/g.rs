use math::factorize;
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
    for (k, mut v) in div {
        let start = (m + k - 1) / k * k;
        let mut i = start - m;
        while v > 0 && i < hi.len() {
            while v > 0 && hi[i] % k == 0 {
                v -= 1;
                hi[i] /= k;
            }
            i += k;
        }
    }

    let mut map = std::collections::HashMap::new();
    for hi in hi {
        let v = factorize(hi as u64);
        for v in v {
            *map.entry(v).or_insert(0) += 1;
        }
    }

    let mut res = 1;
    for (_, v) in map {
        res *= v + 1;
        res %= MOD;
    }

    println!("{}", res);
}
