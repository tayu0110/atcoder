use itertools::Itertools;
use proconio::*;
use rand::{thread_rng, Rng};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, VecDeque},
};

fn solve2(n: usize, k: usize, a: &[usize]) -> usize {
    {
        let mut a = a.to_vec();
        a.dedup();

        if a.len() == n {
            let mut res = 0;
            for v in a.windows(2) {
                res += (v[1] + k - v[0]) % k;
            }

            let mut now = res;
            for i in (1..n).rev() {
                let prev = (i + 1) % n;
                now += (a[prev] + k - a[i]) % k;
                now -= (a[i] + k - a[i - 1]) % k;
                res = res.max(now);
            }
            return res;
        }
    }

    let mut map = BTreeMap::new();
    for &a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut t = vec![];
    while !map.is_empty() {
        let mut keys = map.keys().rev().cloned().collect::<VecDeque<_>>();
        for &k in &keys {
            *map.entry(k).or_insert(0) -= 1;
            if *map.get(&k).unwrap() == 0 {
                map.remove(&k);
            }
        }

        t.push(keys);
    }

    let mut buf = vec![];
    for mut v in t.into_iter().rev() {
        if v.len() == 1 {
            buf.extend(v);
            continue;
        }

        if buf.is_empty() {
            let mut sum = 0;
            for i in 0..v.len() - 1 {
                sum += (v[i + 1] + k - v[i]) % k;
            }

            let mut max = sum;
            let mut p = 0;
            let len = v.len();
            for i in 0..len {
                let back = v.pop_back().unwrap();
                sum += (v[0] + k - back) % k;
                sum -= (back + k - v[len - 2]) % k;
                if sum > max {
                    max = sum;
                    p = i + 1;
                }
                v.push_front(back);
            }

            for _ in 0..p {
                let back = v.pop_back().unwrap();
                v.push_front(back);
            }
            buf.extend(v.into_iter().rev());
        } else {
            let &max = buf.last().unwrap();
            if max < v[0] {
                while let Some(last) = v.pop_back() {
                    if last <= max {
                        v.push_front(last);
                    } else {
                        v.push_back(last);
                        break;
                    }
                }
            }
            buf.extend(v.into_iter().rev());
        }
    }

    buf.reverse();

    let mut res = 0;

    // eprintln!("t: {t:?}");

    for v in buf.windows(2) {
        let d = (v[1] + k - v[0]) % k;
        res += d;
    }

    let mut now = res;
    for i in (1..n).rev() {
        let prev = (i + 1) % n;
        now += (buf[prev] + k - buf[i]) % k;
        now -= (buf[i] + k - buf[i - 1]) % k;
        res = res.max(now);
    }

    res
}

fn solve(n: usize, k: usize, a: &[usize]) -> usize {
    {
        let mut a = a.to_vec();
        a.dedup();

        if a.len() == n {
            let mut res = 0;
            for v in a.windows(2) {
                res += (v[1] + k - v[0]) % k;
            }

            let mut now = res;
            for i in (1..n).rev() {
                let prev = (i + 1) % n;
                now += (a[prev] + k - a[i]) % k;
                now -= (a[i] + k - a[i - 1]) % k;
                res = res.max(now);
            }
            return res;
        }
    }

    let mut map = BTreeMap::new();
    for &a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut t = vec![];
    while !map.is_empty() {
        let mut keys = map.keys().rev().cloned().collect::<VecDeque<_>>();
        for &k in &keys {
            *map.entry(k).or_insert(0) -= 1;
            if *map.get(&k).unwrap() == 0 {
                map.remove(&k);
            }
        }

        if map.is_empty() {
            t.extend(keys);
            continue;
        }

        let &max = map.last_key_value().unwrap().0;
        if max < keys[0] {
            while let Some(last) = keys.pop_back() {
                if last <= max {
                    keys.push_front(last);
                } else {
                    keys.push_back(last);
                    break;
                }
            }
        }
        t.extend(keys);
    }

    let mut res = 0;
    let t = t.into_iter().collect::<Vec<_>>();

    // eprintln!("t: {t:?}");

    for v in t.windows(2) {
        let d = (v[1] + k - v[0]) % k;
        res += d;
    }

    let mut now = res;
    for i in (1..n).rev() {
        let prev = (i + 1) % n;
        now += (t[prev] + k - t[i]) % k;
        now -= (t[i] + k - t[i - 1]) % k;
        res = res.max(now);
    }

    res

    // let mut map = BTreeMultiSet::new();
    // for &a in a {
    //     map.insert(a);
    // }

    // let &max = map.last().unwrap();
    // map.remove(&max);

    // let mut t = VecDeque::new();
    // t.push_back(max);

    // while !map.is_empty() {
    //     let fnext = {
    //         let &front = t.front().unwrap();
    //         let &min = map.first().unwrap();
    //         if let Some(max) = map.range(front + 1..).next() {
    //             if (front + k * 2 - max) % k > (front + k * 2 - min) % k {
    //                 *max
    //             } else {
    //                 min
    //             }
    //         } else {
    //             min
    //         }
    //     };
    //     let bnext = {
    //         let &back = t.back().unwrap();
    //         let &max = map.last().unwrap();
    //         if let Some(min) = map.range(..back).next() {
    //             if (min + k * 2 - back) % k > (max + k * 2 - back) % k {
    //                 *min
    //             } else {
    //                 max
    //             }
    //         } else {
    //             max
    //         }
    //     };

    //     if (t.front().unwrap() + k - fnext) % k > (bnext + k - t.back().unwrap()) % k {
    //         t.push_front(fnext);
    //         map.remove(&fnext);
    //     } else {
    //         t.push_back(bnext);
    //         map.remove(&bnext);
    //     }
    // }

    // let mut res = 0;
    // let t = t.into_iter().collect::<Vec<_>>();

    // eprintln!("t: {t:?}");

    // for v in t.windows(2) {
    //     let d = (v[1] + k - v[0]) % k;
    //     res += d;
    // }

    // res
}

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}
    // let mut rng = thread_rng();
    // for _ in 0..100000000 {
    //     let n = rng.gen_range(1..10);
    //     let k = rng.gen_range(1..100);
    //     let mut a = (0..n).map(|_| rng.gen_range(0..k)).collect::<Vec<_>>();
    //     eprintln!("n: {n}, k: {k}, a: {a:?}");
    a.sort();
    a.reverse();

    for i in 0..n - 1 {
        let d = a[i] - a[i + 1];
        if k - d < d {
            a[i + 1..].iter_mut().for_each(|a| *a += k);
            a.sort_unstable_by_key(|a| Reverse(*a));
            break;
        }
    }

    let mut res = solve(n, k, &a);
    if n < 10 {
        let mut check = 0;
        a.sort();
        let mut keep = vec![];
        for v in a.iter().permutations(n) {
            let mut sum = 0;
            for v in v.windows(2) {
                let d = (v[1] + k - v[0]) % k;
                sum += d;
            }
            check = check.max(sum);
            if sum == check {
                keep.push(v.iter().map(|v| **v).collect::<Vec<_>>());
            }
        }

        res = res.max(check);
    }

    res = res.max(solve2(n, k, &a));
    if cfg!(debug_assertions) {
        let mut check = 0;
        a.sort();
        let mut keep = vec![];
        for v in a.iter().permutations(n) {
            let mut sum = 0;
            for v in v.windows(2) {
                let d = (v[1] + k - v[0]) % k;
                sum += d;
            }
            check = check.max(sum);
            if sum == check {
                keep.push(v.iter().map(|v| **v).collect::<Vec<_>>());
            }
        }

        eprintln!("keep: {:?}", keep.last().unwrap());
        assert_eq!(check, res);
    }

    println!("{}", res)
    // }
}
