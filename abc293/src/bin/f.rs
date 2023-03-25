use math::factorize;
use proconio::*;
use std::collections::{HashMap, HashSet};

fn solve(n: usize, memo: &mut HashSet<u64>) -> usize {
    let mut f = factorize(n as u64);
    f.sort();
    let mut t = vec![];
    for c in f {
        match t.last_mut() {
            Some((d, cnt)) if *d == c => *cnt += 1,
            _ => t.push((c, 1)),
        }
    }

    let mut divs = vec![1];
    for (c, cnt) in t {
        let len = divs.len();
        let mut now = c;
        for _ in 0..cnt {
            for i in 0..len {
                let new = divs[i] * now;
                divs.push(new);
            }
            now = now.wrapping_mul(c);
        }
    }

    let mut res = 0;
    for d in divs {
        if d > 1 && !memo.contains(&d) {
            let mut k = n as u64;
            while k > 0 {
                if k % d > 1 {
                    break;
                }
                k /= d;
            }

            if k == 0 {
                res += 1;
            }
        }
        memo.insert(d);
    }
    res
}

fn main() {
    input! {t: usize}

    let mut cache = HashMap::new();
    for _ in 0..t {
        input! {n: usize}

        if cache.contains_key(&n) {
            println!("{}", cache.get(&n).unwrap());
            continue;
        }

        let mut memo = HashSet::new();
        let res = solve(n, &mut memo) + solve(n - 1, &mut memo);
        println!("{}", res);

        cache.insert(n, res);
    }
}
