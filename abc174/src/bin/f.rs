use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, c: [usize; n], p: [(usize, usize); q]}

    let mut query = p
        .into_iter()
        .enumerate()
        .map(|(i, (l, r))| (r, l - 1, i))
        .collect::<Vec<_>>();
    let nq = ((3f64.sqrt() * n as f64) / ((2 * q) as f64).sqrt()).round() as usize;
    query.sort_by(|&l, &r| {
        let (bl, br) = (l.0 / nq, r.0 / nq);
        if bl == br {
            if bl & 1 == 1 {
                r.1.cmp(&l.1)
            } else {
                l.1.cmp(&r.1)
            }
        } else {
            bl.cmp(&br)
        }
    });

    let (mut back, mut top) = (0, 0);
    let mut used = vec![0; n + 1];
    let mut cnt = 0;
    let mut res = vec![0; q];
    for (r, l, i) in query {
        while top < r {
            used[c[top]] += 1;
            if used[c[top]] == 1 {
                cnt += 1;
            }
            top += 1;
        }
        while r < top {
            top -= 1;
            used[c[top]] -= 1;
            if used[c[top]] == 0 {
                cnt -= 1;
            }
        }
        while l < back {
            back -= 1;
            used[c[back]] += 1;
            if used[c[back]] == 1 {
                cnt += 1;
            }
        }
        while back < l {
            used[c[back]] -= 1;
            if used[c[back]] == 0 {
                cnt -= 1;
            }
            back += 1;
        }

        res[i] = cnt;
    }

    println!("{}", res.iter().join("\n"))
}
