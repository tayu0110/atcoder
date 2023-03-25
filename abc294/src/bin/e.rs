use proconio::*;

fn main() {
    input! {_l: usize, n: usize, m: usize, p: [(usize, usize); n], q: [(usize, usize); m]}

    let mut t = vec![];
    for v in vec![&p, &q] {
        let mut now = 0;
        for &(_, l) in v {
            now += l;
            t.push(now);
        }
    }

    t.sort();
    t.dedup();

    let p = {
        let mut now = 0;
        let mut k = 0;
        let mut np = vec![];
        for &(v, l) in &p {
            let a = k + l;
            while now < t.len() && t[now] <= a {
                np.push((v, t[now] - k));
                k = t[now];
                now += 1;
            }
        }
        np
    };
    let q = {
        let mut now = 0;
        let mut k = 0;
        let mut np = vec![];
        for &(v, l) in &q {
            let a = k + l;
            while now < t.len() && t[now] <= a {
                np.push((v, t[now] - k));
                k = t[now];
                now += 1;
            }
        }
        np
    };

    assert_eq!(p.len(), q.len());

    let mut res = 0;
    for ((v1, l1), (v2, l2)) in p.into_iter().zip(q) {
        assert_eq!(l1, l2);

        if v1 == v2 {
            res += l1;
        }
    }

    println!("{}", res);
}
