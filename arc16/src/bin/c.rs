use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut v = vec![];
    for _ in 0..m {
        input! {c: usize, cost: f64}

        let mut t = vec![];
        for _ in 0..c {
            input! {idol: usize, p: usize}
            t.push((idol - 1, p));
        }

        v.push((cost, t));
    }

    let mut res = vec![std::f64::MAX; 1 << n];
    res[0] = 0.0;
    for i in 1..1 << n {
        for (cost, t) in &v {
            let mut np = 0;
            let mut c = *cost;
            for &(idol, p) in t {
                if i & (1 << idol) != 0 {
                    c += res[i ^ (1 << idol)] * p as f64 / 100.0;
                } else {
                    np += p;
                }
            }

            if np < 100 && c / (1.0 - np as f64 / 100.0) < res[i] {
                res[i] = c / (1.0 - np as f64 / 100.0);
            }
        }
    }

    println!("{}", res[(1 << n) - 1]);
}
