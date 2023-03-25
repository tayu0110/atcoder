use num::integer::Roots;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, a: [usize; n], p: [(usize, usize); q]}

    let backet_size = q.sqrt();
    let mut p = p
        .into_iter()
        .enumerate()
        .map(|(i, (l, r))| (l, r, i))
        .collect::<Vec<_>>();
    p.sort_by(|&(l, r, _), &(nl, nr, _)| {
        if l / backet_size != nl / backet_size {
            l.cmp(&nl)
        } else {
            r.cmp(&nr)
        }
    });

    let f = |t: i64| t * (t - 1) / 2;
    let mut res = vec![0i64; q];
    let mut map = vec![0i64; 2 * 100000 + 1];
    {
        let (l, r, i) = p[0];
        for j in l - 1..r {
            res[i] += f(map[a[j]]);
            map[a[j]] += 1;
        }
    };
    for v in p.windows(2) {
        let (pl, pr, i) = v[0];
        let (l, r, j) = v[1];
        res[j] = res[i];
        if l < pl {
            for k in l - 1..pl - 1 {
                res[j] += f(map[a[k]]);
                map[a[k]] += 1;
            }
        } else if l > pl {
            for k in pl - 1..l - 1 {
                map[a[k]] -= 1;
                res[j] -= f(map[a[k]]);
            }
        }

        if r < pr {
            for k in r..pr {
                map[a[k]] -= 1;
                res[j] -= f(map[a[k]]);
            }
        } else if r > pr {
            for k in pr..r {
                res[j] += f(map[a[k]]);
                map[a[k]] += 1;
            }
        }
    }

    for i in 0..q {
        println!("{}", res[i]);
    }
}
