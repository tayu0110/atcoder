use proconio::input;

fn main() {
    input! {n: usize, k: i64, p: [usize; n], c: [i64; n]}

    let mut t = vec![];
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        if set.contains(&i) {
            continue;
        }

        let mut buf = vec![];
        let mut now = i;
        while !set.contains(&now) {
            set.insert(now);
            buf.push(c[now]);
            now = p[now] - 1;
        }

        t.push(buf);
    }

    let mut res = c[0];
    for v in t {
        let sum: i64 = v.iter().sum();

        let len = v.len();
        let mut v: Vec<_> = vec![0].into_iter().chain(v.repeat(2).into_iter()).collect();
        for i in 1..v.len() {
            v[i] += v[i - 1];
        }
        if sum >= 0 {
            let r = sum * (k / len as i64);
            if k / len as i64 >= 1 {
                res = std::cmp::max(res, r);
                for k in 1..=len {
                    for i in 0..len {
                        res = std::cmp::max(res, r - sum + v[i + k as usize] - v[i]);
                    }
                }
            }
            let nk = k % len as i64;
            for k in 1..=nk {
                for i in 0..len {
                    res = std::cmp::max(res, r + v[i + k as usize] - v[i]);
                }
            }
        } else {
            for k in 1..=std::cmp::min(len, k as usize) {
                for i in 0..len {
                    res = std::cmp::max(res, v[i + k as usize] - v[i]);
                }
            }
        }
    }

    println!("{}", res);
}
