use itertools::Itertools;
use math::MathInt;
use proconio::*;
use rustc_hash::FxHashMap;

const MAX: usize = 10_000_000;
const M: usize = 998244353;

fn main() {
    input! {t: usize}

    let mut memo = vec![0u32; MAX + 1];
    memo[1] = 1;
    for i in 2..MAX {
        if memo[i] == 0 {
            for j in (1..).take_while(|&j| j * i <= MAX) {
                memo[i * j] = i as u32;
            }
        }
    }
    let mut map = FxHashMap::default();
    let mut div = vec![];
    let mut count = vec![];
    let mut s = vec![];
    for _ in 0..t {
        input! {n: usize, a: [usize; n]}
        map.clear();
        s.clear();

        for &(mut a) in &a {
            div.clear();
            while a > 1 {
                div.push(memo[a]);
                a /= memo[a] as usize;
            }
            div.sort_unstable();
            count.clear();

            for &d in &div {
                match count.last_mut() {
                    Some((p, cnt)) if *p == d => *cnt += 1,
                    _ => count.push((d, 1)),
                }
            }

            for &(d, cnt) in &count {
                let p = map.entry(d).or_insert((0, 0));
                if p.0 < cnt {
                    p.1 = p.0;
                    p.0 = cnt;
                } else if p.1 < cnt {
                    p.1 = cnt;
                }
            }

            s.push(count.clone());
        }

        let mut ret = 1usize;
        for (&k, &v) in &map {
            ret *= k.pow_mod(v.0, M as u32) as usize;
            ret %= M;
        }

        let mut buf = vec![];
        for count in s.drain(..) {
            let mut ret = ret;
            for (p, cnt) in count {
                let &(max, second) = map.get(&p).unwrap();
                if cnt == max {
                    ret *= p
                        .pow_mod(max - second, M as u32)
                        .inverse_mod(M as u32)
                        .unwrap() as usize;
                    ret %= M;
                }
            }

            buf.push(ret);
        }

        println!("{}", buf.iter().join(" "));
    }
}
