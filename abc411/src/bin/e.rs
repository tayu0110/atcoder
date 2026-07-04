use math::MathInt;
use proconio::*;
use rustc_hash::FxHashMap;

const M: usize = 998244353;

fn main() {
    input! {n: usize, mut a: [[usize; 6]; n]}

    let &min = a.iter().map(|a| a.iter().min().unwrap()).max().unwrap();
    let mut res = 0;
    let mut mul = 1;
    let mut map = FxHashMap::default();
    for (i, a) in a.iter_mut().enumerate() {
        mul *= a.iter().filter(|&&a| a <= min).count();
        mul %= M;

        a.sort();
        let mut t = vec![];
        for a in a {
            match t.last_mut() {
                Some((p, cnt)) if *p == *a => *cnt += 1,
                _ => t.push((*a, 1)),
            }
        }

        for (t, cnt) in t {
            map.entry(t).or_insert(vec![]).push((cnt, i));
        }
    }

    let base = 6usize.pow_mod(n as u64, M).inverse_mod(M).unwrap();
    res += mul * min % M * base % M;

    let mut t = a.iter().flatten().copied().collect::<Vec<_>>();
    t.sort_unstable();
    t.dedup();

    let mut now = vec![0; n];
    for t in t {
        if t <= min {
            let cnt = map.remove(&t).unwrap();
            for (cnt, i) in cnt {
                now[i] += cnt;
            }
            continue;
        }

        let prev = mul;
        let cnt = map.remove(&t).unwrap();
        for (cnt, i) in cnt {
            let rem = mul * now[i].inverse_mod(M).unwrap() % M;
            now[i] += cnt;
            mul = rem * now[i] % M;
        }
        res += (mul + M - prev) % M * t % M * base % M;
        res %= M;
    }

    println!("{res}")
}
