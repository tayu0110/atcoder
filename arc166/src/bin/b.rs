use proconio::*;

fn solve(a: &[usize], h: Vec<usize>) -> usize {
    let n = a.len();
    let mut t = vec![];
    for s in h {
        let mut buf = vec![];
        for (i, &a) in a.iter().enumerate() {
            let tar = (a + s - 1) / s * s;
            buf.push((tar - a, i));
        }
        buf.sort();
        t.push(buf);
    }

    let mut res = usize::MAX;
    for i in 0..100.min(n) {
        let (p, i) = t[0][i];
        if t.len() == 1 {
            res = res.min(p);
        } else {
            for j in 0..100.min(n) {
                let (q, j) = t[1][j];
                if i == j {
                    continue;
                }
                if t.len() == 2 {
                    res = res.min(p + q);
                } else {
                    for k in 0..100.min(n) {
                        let (r, k) = t[2][k];

                        if i == j || j == k || k == i {
                            continue;
                        }

                        res = res.min(p + q + r);
                    }
                }
            }
        }
    }
    res
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        (x, y) = (y, x);
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    x / gcd(x, y) * y
}

fn main() {
    input! {n: usize, p: usize, q: usize, r: usize, a: [usize; n]}

    let mut res = solve(&a, vec![p, q, r]);
    res = res.min(solve(&a, vec![lcm(p, q), r]));
    res = res.min(solve(&a, vec![p, lcm(q, r)]));
    res = res.min(solve(&a, vec![q, lcm(p, r)]));
    res = res.min(solve(&a, vec![lcm(p, lcm(q, r))]));

    println!("{}", res)
}
